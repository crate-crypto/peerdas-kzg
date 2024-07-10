use bls12_381::ff::Field;
use bls12_381::{batch_inversion::batch_inverse, Scalar};

use crate::errors::DecodeError;
use polynomial::{domain::Domain, monomial::vanishing_poly};

// The erasures can be either indices of the polynomial
// or groups of indices
#[derive(Debug, Clone)]
pub struct Erasures {
    pub coset_size: usize,
    pub cosets: Vec<usize>,
}

#[derive(Debug)]
pub struct ReedSolomon {
    expansion_factor: usize,
    poly_len: usize,
    evaluation_domain: Domain,
}

impl ReedSolomon {
    pub fn new(poly_len: usize, expansion_factor: usize) -> Self {
        let evaluation_domain = Domain::new(poly_len * expansion_factor);
        Self {
            poly_len,
            evaluation_domain,
            expansion_factor,
        }
    }

    /// We need to have at least `poly_len` evaluations
    pub fn acceptable_num_errors(&self) -> usize {
        let total_codeword_len = self.poly_len * self.expansion_factor;
        let min_num_evaluations_needed = self.poly_len;
        total_codeword_len - min_num_evaluations_needed
    }

    /// The number of scalars in the reed solomon encoded polynomial
    pub fn extended_polynomial_length(&self) -> usize {
        self.poly_len * self.expansion_factor
    }

    /// Reed solomon encodes a polynomial by evaluating it at `expansion_factor`
    /// more points than is needed.
    pub fn encode(&self, poly_coefficient_form: Vec<Scalar>) -> Vec<Scalar> {
        if poly_coefficient_form.len() > self.poly_len {
            panic!(
                "The polynomial must have a size of {}, found {}",
                self.poly_len,
                poly_coefficient_form.len()
            )
        }
        self.evaluation_domain.fft_scalars(poly_coefficient_form)
    }

    pub fn recover_polynomial_codeword(
        &self,
        codeword_with_errors: Vec<Scalar>,
        missing_indices: Erasures,
    ) -> Vec<Scalar> {
        recover_polynomial_evaluations(
            &self.evaluation_domain,
            codeword_with_errors,
            missing_indices,
        )
    }

    pub fn recover_polynomial_coefficient(
        &self,
        codeword_with_errors: Vec<Scalar>,
        missing_indices: Erasures,
    ) -> Result<Vec<Scalar>, DecodeError> {
        let coefficients = recover_polynomial_coefficient(
            &self.evaluation_domain,
            codeword_with_errors,
            missing_indices,
        );

        // Check that the polynomial being returned has the correct degree
        //
        // This is the polynomial in coefficient form that corresponds to the
        // data in lagrange form being returned. This means this polynomial will
        // have the same number of coefficients as the original data.
        //
        // All of the coefficients after the original data should be zero.
        for (i, coefficient) in coefficients.iter().enumerate().skip(self.poly_len) {
            if *coefficient != Scalar::ZERO {
                return Err(DecodeError::PolynomialHasInvalidLength {
                    num_coefficients: i,
                    expected_num_coefficients: self.poly_len,
                });
            }
        }

        // Return the truncated polynomial
        Ok(coefficients[0..self.poly_len].to_vec())
    }
}

/// Given a set of evaluations and a list of its erasures,
/// This method will return the polynomial in coefficient form
/// with the missing indices filled in (recovered).
fn recover_polynomial_coefficient(
    evaluation_domain: &Domain,
    data_eval: Vec<Scalar>,
    missing_indices: Erasures,
) -> Vec<Scalar> {
    // Compute Z(X) which is the polynomial that vanishes on all
    // of the missing points
    let z_x = construct_vanishing_poly_from_erasures(missing_indices, evaluation_domain);

    // Compute Z(X)_eval which is the vanishing polynomial evaluated
    // at the missing points
    let z_x_eval = evaluation_domain.fft_scalars(z_x.clone());

    assert_eq!(
        z_x_eval.len(),
        data_eval.len(),
        "incorrect length for encoded data, expected {}, found {}",
        z_x_eval.len(),
        data_eval.len()
    );
    // Compute (D * Z)(X) or (E * Z)(X) (same polynomials)
    let ez_eval: Vec<_> = z_x_eval
        .iter()
        .zip(data_eval)
        .map(|(zx, d)| zx * d)
        .collect();

    let dz_poly = evaluation_domain.ifft_scalars(ez_eval);

    let mut coset_z_x_eval = evaluation_domain.coset_fft_scalars(z_x);
    let coset_dz_eval = evaluation_domain.coset_fft_scalars(dz_poly);
    // We know that none of the values will be zero since we are evaluating z_x
    // over a coset, that we know it has no roots in.
    batch_inverse(&mut coset_z_x_eval);
    let coset_quotient_eval: Vec<_> = coset_dz_eval
        .iter()
        .zip(coset_z_x_eval)
        .map(|(d, zx_inv)| d * zx_inv)
        .collect();

    evaluation_domain.coset_ifft_scalars(coset_quotient_eval)
}

fn recover_polynomial_evaluations(
    evaluation_domain: &Domain,
    evaluations: Vec<Scalar>,
    missing_indices: Erasures,
) -> Vec<Scalar> {
    let polynomial_coeff =
        recover_polynomial_coefficient(evaluation_domain, evaluations, missing_indices);

    evaluation_domain.fft_scalars(polynomial_coeff)
}

fn construct_vanishing_poly_from_erasures(
    erasures: Erasures,
    evaluation_domain: &Domain,
) -> Vec<Scalar> {
    let cosets = erasures.cosets;
    let coset_size = erasures.coset_size;

    let evaluation_domain_size = evaluation_domain.roots.len();
    let num_cosets_per_extended_polynomial = evaluation_domain_size / coset_size;
    let domain = Domain::new(num_cosets_per_extended_polynomial);

    let z_x_missing_indices_roots: Vec<_> =
        cosets.iter().map(|index| domain.roots[*index]).collect();
    let short_zero_poly = vanishing_poly(&z_x_missing_indices_roots);

    let mut z_x = vec![Scalar::ZERO; evaluation_domain_size];
    for (i, coeff) in short_zero_poly.into_iter().enumerate() {
        z_x[i * coset_size] = coeff;
    }
    z_x
}

#[test]
fn smoke_test_recovery_no_errors() {
    let rs = ReedSolomon::new(16, 2);
    let poly_coeff: Vec<_> = (0..15).map(|i| -Scalar::from(i)).collect();

    let codewords = rs.encode(poly_coeff);
    assert_eq!(codewords.len(), 32);
    let got_codewords = rs.recover_polynomial_codeword(
        codewords.clone(),
        Erasures {
            coset_size: 64,
            cosets: Vec::new(),
        },
    );

    assert_eq!(got_codewords, codewords);
}

// #[test]
// fn smoke_test_recovery_upto_num_acceptable_errors() {
//     let poly_len = 16;
//     let expansion_factor = 2;
//     let rs = ReedSolomon::new(poly_len, expansion_factor);
//     let poly_coeff = (0..poly_len)
//         .map(|i| Scalar::from(i as u64))
//         .collect::<Vec<_>>();

//     let original_codewords = rs.encode(poly_coeff);
//     let acceptable_num_errors: Vec<_> = (0..rs.acceptable_num_errors()).collect();
//     for num_errors in acceptable_num_errors {
//         let mut codewords_with_errors = original_codewords.clone();

//         // zero out `num_errors` amount of evaluations to simulate errors
//         let mut missing_indices = Vec::new();
//         for index in 0..num_errors {
//             codewords_with_errors[index] = Scalar::from(0);
//             missing_indices.push(index);
//         }

//         let recovered_codewords = rs
//             .recover_polynomial_codeword(codewords_with_errors, Erasures::Indices(missing_indices));
//         assert_eq!(recovered_codewords, original_codewords)
//     }
// }
