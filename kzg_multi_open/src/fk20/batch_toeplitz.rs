use crate::fk20::toeplitz::{CirculantMatrix, ToeplitzMatrix};
use bls12_381::{fixed_base_msm::FixedBaseMSM, g1_batch_normalize, G1Point, G1Projective};
use polynomial::domain::Domain;
use rayon::prelude::*;

/// BatchToeplitz is a structure that optimizes for the usecase where:
/// - You need to do multiple matrix-vector multiplications and sum them together
/// - The vector is known at compile time, so you can precompute it's FFT
/// - For now, the vector is a group element. We don't have any other usecases in the codebase.
#[derive(Debug)]
pub struct BatchToeplitzMatrixVecMul {
    /// This contains the number of matrix-vector multiplications that
    /// we can do in a batch.
    batch_size: usize,
    precomputed_fft_vectors: Vec<FixedBaseMSM>,
    // This is the length of the vector that we are multiplying the matrix with.
    // and subsequently will be the length of the final result of the matrix-vector multiplication.
    n: usize,
    /// This is the domain used in the circulant matrix-vector multiplication.
    /// It will be double the size of the length of the vector.
    circulant_domain: Domain,
}

impl BatchToeplitzMatrixVecMul {
    pub fn new(vectors: Vec<Vec<G1Point>>) -> Self {
        let n = vectors[0].len();
        let vectors_all_same_length = vectors.iter().all(|v| v.len() == n);
        assert!(
            vectors_all_same_length,
            "expected all vectors to be the same length"
        );

        let circulant_domain = Domain::new(n * 2);

        // Precompute the FFT of the vectors
        let vectors: Vec<Vec<G1Point>> = vectors
            .into_par_iter()
            .map(|vector| {
                let vector_projective = vector
                    .iter()
                    .map(|point| G1Projective::from(*point))
                    .collect::<Vec<_>>();
                g1_batch_normalize(&circulant_domain.fft_g1(vector_projective))
            })
            .collect();
        let batch_size = vectors.len();

        let transposed_msm_vectors = transpose(vectors);

        let table_bits = 8;
        let precomputed_table: Vec<_> = transposed_msm_vectors
            .into_par_iter()
            .map(|v| FixedBaseMSM::new(v, table_bits))
            .collect();

        BatchToeplitzMatrixVecMul {
            n,
            circulant_domain,
            precomputed_fft_vectors: precomputed_table,
            batch_size,
        }
    }

    // Computes the aggregated sum of many Toeplitz matrix-vector multiplications.
    //
    // ie this method computes \sum_{i}^{n} A_i* x_i
    //
    // This is faster than computing the matrix vector multiplication for each Toeplitz matrix and then summing the results
    // since only one IFFT is done as opposed to `n`
    // TODO: This method should be refactored for better readability, once we are done applying optimizations
    pub fn sum_matrix_vector_mul(&self, matrices: Vec<ToeplitzMatrix>) -> Vec<G1Projective> {
        assert_eq!(
            matrices.len(),
            self.batch_size,
            "expected the number of matrices to be the same as the number of vectors"
        );

        // Embed Toeplitz matrices into Circulant matrices
        let circulant_matrices = matrices.into_iter().map(CirculantMatrix::from_toeplitz);

        // Perform circulant matrix-vector multiplication between all of the matrices and vectors
        // and sum them together.
        //
        // We note that the aggregation step can be converted into msm's of size `l`
        let col_ffts: Vec<_> = circulant_matrices
            .into_iter()
            .map(|matrix| self.circulant_domain.fft_scalars(matrix.row))
            .collect();
        let msm_scalars = transpose(col_ffts);

        let result: Vec<_> = self
            .precomputed_fft_vectors
            .par_iter()
            .zip(msm_scalars)
            .map(|(points, scalars)| points.msm(scalars))
            .collect();

        // Once the Circulant matrix-vector multiplication is done, we need to take the first half
        // of the result, as this is the result of the Toeplitz matrix multiplication
        self.circulant_domain.ifft_g1_take_n(result, Some(self.n))
    }
}

/// Transposes a 2D matrix
///
/// This function takes a vector of vectors (representing a matrix) and returns its transpose,
/// ie a new matrix whose rows are the columns of the original.
///
/// # Examples
///
/// ```text
/// matrix = [
///     [1, 2, 3],
///     [4, 5, 6]
/// ];
/// Transpose will produce the following matrix:
///
/// [
///     [1, 4],
///     [2, 5],
///     [3, 6]
/// ]
/// ```
pub(crate) fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() || v[0].is_empty() {
        return Vec::new();
    }

    let rows = v.len();
    let cols = v[0].len();

    let mut result = vec![Vec::with_capacity(rows); cols];

    for row in v {
        for (i, elem) in row.into_iter().enumerate() {
            result[i].push(elem);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::fk20::batch_toeplitz::BatchToeplitzMatrixVecMul;
    use crate::fk20::toeplitz::ToeplitzMatrix;
    use bls12_381::group::Group;
    use bls12_381::{g1_batch_normalize, G1Projective, Scalar};

    #[test]
    fn smoke_aggregated_matrix_vector_mul() {
        // Create the toeplitz matrices and vectors that we want to perform matrix-vector multiplication with
        let mut toeplitz_matrices = Vec::new();
        let mut vectors = Vec::new();
        let mut vectors_affine = Vec::new();

        let num_matrices = 10;
        for i in 0..num_matrices {
            let col = vec![
                Scalar::from((i + 1) as u64),
                Scalar::from((i + 2) as u64),
                Scalar::from((i + 3) as u64),
                Scalar::from((i + 4) as u64),
            ];
            let row = vec![
                Scalar::from((i + 1) as u64),
                Scalar::from((i + 5) as u64),
                Scalar::from((i + 6) as u64),
                Scalar::from((i + 7) as u64),
            ];
            let vector = vec![
                G1Projective::generator() * Scalar::from((i + 1) as u64),
                G1Projective::generator() * Scalar::from((i + 2) as u64),
                G1Projective::generator() * Scalar::from((i + 3) as u64),
                G1Projective::generator() * Scalar::from((i + 4) as u64),
            ];

            vectors_affine.push(g1_batch_normalize(&vector.clone()));
            vectors.push(vector);
            toeplitz_matrices.push(ToeplitzMatrix::new(row, col));
        }

        let bm = BatchToeplitzMatrixVecMul::new(vectors_affine);
        let got_result = bm.sum_matrix_vector_mul(toeplitz_matrices.clone());

        let mut expected_result = vec![G1Projective::identity(); got_result.len()];
        for (matrix, vector) in toeplitz_matrices.into_iter().zip(vectors) {
            let intermediate_result = matrix.vector_mul_g1(vector);
            for (got, expected) in expected_result.iter_mut().zip(intermediate_result) {
                *got += expected;
            }
        }

        assert_eq!(expected_result, got_result)
    }
}
