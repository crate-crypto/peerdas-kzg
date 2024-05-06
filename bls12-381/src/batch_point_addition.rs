use crate::batch_inversion::batch_inverse;
use blstrs::G1Affine;

/// Adds multiple points together in affine representation, batching the inversions
pub fn batch_addition(mut points: Vec<G1Affine>) -> G1Affine {
    fn point_add(p1: G1Affine, p2: G1Affine, inv: &blstrs::Fp) -> G1Affine {
        use ff::Field;

        let lambda = (p2.y() - p1.y()) * inv;
        let x = lambda.square() - p1.x() - p2.x();
        let y = lambda * (p1.x() - x) - p1.y();
        G1Affine::from_raw_unchecked(x, y, false)
    }

    if points.is_empty() {
        use group::prime::PrimeCurveAffine;
        return G1Affine::identity();
    }

    let mut stride = 1;

    let mut new_differences = Vec::with_capacity(points.len());

    while stride < points.len() {
        new_differences.clear();

        for i in (0..points.len()).step_by(stride * 2) {
            if i + stride < points.len() {
                new_differences.push(points[i + stride].x() - points[i].x());
            }
        }
        batch_inverse(&mut new_differences);
        for (i, inv) in new_differences.iter().enumerate() {
            let p1 = points[i * stride * 2];
            let p2 = points[i * stride * 2 + stride];
            points[i * stride * 2] = point_add(p1, p2, inv);
        }

        stride *= 2;
    }

    points[0]
}

// Similar to batch addition, however we amortize across different batches
// TODO: Clean up -- This has a greater complexity than the regular algorithm
// TODO so we want to check if it makes a difference in our usecase.
pub fn multi_batch_addition(mut multi_points: Vec<Vec<G1Affine>>) -> Vec<G1Affine> {
    fn point_add(p1: G1Affine, p2: G1Affine, inv: &blstrs::Fp) -> G1Affine {
        use ff::Field;

        let lambda = (p2.y() - p1.y()) * inv;
        let x = lambda.square() - p1.x() - p2.x();
        let y = lambda * (p1.x() - x) - p1.y();
        G1Affine::from_raw_unchecked(x, y, false)
    }

    // Find the largest buckets, this will be the bottleneck for the number of iterations
    let mut max_bucket_length = 0;
    for i in 0..multi_points.len() {
        max_bucket_length = std::cmp::max(max_bucket_length, multi_points[i].len());
    }

    let mut new_differences = Vec::with_capacity(max_bucket_length);
    // (a,b) ; a is the length before adding points and b is the length after adding points. so a range
    let mut collected_points = vec![(0, 0); multi_points.len()]; // We want to know how many points each bucket has accumulated
    let mut multi_strides = vec![1; multi_points.len()]; // We want to know the stride for each bucket
    let mut bucket_complete = vec![false; multi_points.len()]; // We want to know if a bucket is complete
                                                               // Iterate over each bucket

    let max_num_iterations = max_bucket_length.next_power_of_two().ilog2();
    for _ in 0..max_num_iterations {
        new_differences.clear();
        // Iterate over each bucket
        for i in 0..multi_points.len() {
            if bucket_complete[i] {
                continue;
            }
            let points = &multi_points[i];
            let stride = multi_strides[i];
            let old_diff_len = new_differences.len();

            // Skip the bucket if the stride is too long,
            // This happens if the buckets are not evenly distributed
            // in terms of points.
            if stride < points.len() {
                for k in (0..points.len()).step_by(stride * 2) {
                    if k + stride < points.len() {
                        new_differences.push(points[k + stride].x() - points[k].x());
                    }
                }

                // Save the number of points going into this bucket for the batch inversion
                collected_points[i] = (old_diff_len, new_differences.len());
            } else {
                collected_points[i] = (old_diff_len, new_differences.len());
                bucket_complete[i] = true;
            }
        }

        // We have iterated over each bucket, so now we need to do a batch inversion
        batch_inverse(&mut new_differences);
        // Now we update each bucket using the batch inversion we have computed and the collected points
        for i in 0..multi_points.len() {
            if bucket_complete[i] {
                continue;
            }
            let points = &mut multi_points[i];
            let stride = multi_strides[i];
            let (start, end) = collected_points[i];
            for (k, new_difference_offset) in (start..end).enumerate() {
                let inv = &new_differences[new_difference_offset];
                let p1 = points[k * stride * 2];
                let p2 = points[k * stride * 2 + stride];
                points[k * stride * 2] = point_add(p1, p2, inv);
            }

            // Update the stride for this bucket
            multi_strides[i] *= 2;
        }
    }

    multi_points.into_iter().map(|points| points[0]).collect()
}

#[cfg(test)]
mod tests {

    use super::{batch_addition, multi_batch_addition};
    use blstrs::{G1Affine, G1Projective};
    use group::Group;

    #[test]
    fn test_batch_addition() {
        let num_points = 100;
        let points: Vec<G1Affine> = (0..num_points)
            .map(|_| G1Projective::random(&mut rand::thread_rng()).into())
            .collect();

        let expected_result: G1Affine = points
            .iter()
            .fold(G1Projective::identity(), |acc, p| acc + p)
            .into();

        let got_result = batch_addition(points.clone());
        assert_eq!(expected_result, got_result);
    }

    #[test]
    fn test_multi_batch_addition() {
        let num_points = 100;
        let num_sets = 5;
        let random_sets_of_points: Vec<Vec<G1Affine>> = (0..num_sets)
            .map(|_| {
                (0..num_points)
                    .map(|_| G1Projective::random(&mut rand::thread_rng()).into())
                    .collect()
            })
            .collect();
        let random_sets_of_points_clone = random_sets_of_points.clone();

        let expected_results: Vec<_> = random_sets_of_points
            .into_iter()
            .map(|points| batch_addition(points))
            .collect();

        let got_results = multi_batch_addition(random_sets_of_points_clone);
        assert_eq!(got_results, expected_results);
    }
}