use ark_ff::{Field, Zero};
use ark_poly::multivariate::Term;
use ark_poly::multivariate::{SparsePolynomial, SparseTerm};
use ark_poly::MVPolynomial;

pub trait TermEvaluation<FF> {
    fn selected_evaluation(&self, points: &[Option<FF>]) -> (FF, SparseTerm);
}


impl<FF: Field> TermEvaluation<FF> for SparseTerm {
    fn selected_evaluation(&self, points: &[Option<FF>]) -> (FF, SparseTerm) {
        let mut scalar = FF::one();
        let mut term = vec![];

        for (var, power) in self.iter() {
            match points.get(*var) {
                // If the point is zero, the whole term evaluates to zero
                Some(Some(point)) if point.is_zero() => {
                    return (FF::zero(), SparseTerm::new(vec![]));
                }
                // If the point is provided, raise it to the power and multiply
                Some(Some(point)) => {
                    scalar *= point.pow([*power as u64]);
                }
                // If the point is missing, keep the variable in the term
                _ => term.push((*var, *power)),
            }
        }

        (scalar, SparseTerm::new(term))
    }
}


pub trait MultivariateEvaluation<F> {
    /// Only evaluate selected variables
    fn selected_evaluation(&self, points: &[Option<F>]) -> Self;
}


impl<FF: Field> MultivariateEvaluation<FF> for SparsePolynomial<FF, SparseTerm> {
    fn selected_evaluation(&self, points: &[Option<FF>]) -> Self {
        assert!(
            points.len() <= self.num_vars(),
            "Invalid number of variables"
        );

        if self.is_zero() {
            return self.clone();
        }

        let terms: Vec<(FF, SparseTerm)> = self
            .terms
            .iter()
            .map(|(coeff, term)| {
                let (new_coeff, new_term) = term.selected_evaluation(points);
                let new_coeff = *coeff * new_coeff;
                (new_coeff, new_term)
            })
            .collect();

        SparsePolynomial::from_coefficients_vec(self.num_vars(), terms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr; // Use Fr from the ark_bn254 crate as the field type
    use ark_ff::One;
    use ark_poly::multivariate::{SparseTerm, Term};

    
    #[test]
    fn test_selected_evaluation_full_points() {
        // Example SparseTerm: x_1^2 * x_2^3
        let term = SparseTerm::new(vec![(1, 2), (2, 3)]);
        let points: Vec<Option<Fr>> = vec![
            Some(Fr::from(1u64)), // Corresponds to x_0 (which isn't used in this term)
            Some(Fr::from(2u64)), // Corresponds to x_1, where power is 2
            Some(Fr::from(3u64)), // Corresponds to x_2, where power is 3
        ];

        // Evaluate the term at the given points
        let (result_scalar, result_term) = term.selected_evaluation(&points);

        // Print intermediate results
        println!("Result scalar: {:?}", result_scalar);
        println!("Result term: {:?}", result_term);

        // Manually calculate the expected result: 2^2 * 3^3 in the Fr field
        let expected_scalar_2_pow_2 = Fr::from(2u64).pow([2]);
        let expected_scalar_3_pow_3 = Fr::from(3u64).pow([3]);
        let expected_scalar = expected_scalar_2_pow_2 * expected_scalar_3_pow_3;

        println!("Expected scalar (2^2): {:?}", expected_scalar_2_pow_2);
        println!("Expected scalar (3^3): {:?}", expected_scalar_3_pow_3);
        println!("Combined expected scalar (2^2 * 3^3): {:?}", expected_scalar);

        let expected_term = SparseTerm::new(vec![]); // No remaining term

        // Assert the scalar matches expected value
        assert_eq!(result_scalar, expected_scalar, "Scalar mismatch");
        assert_eq!(result_term, expected_term, "Term mismatch");
    }
    

    // #[test]
    // fn test_selected_evaluation_full_points() {
    //     // Example SparseTerm: x_1^2 * x_2^3
    //     let term = SparseTerm::new(vec![(1, 2), (2, 3)]);
    //     let points: Vec<Option<Fr>> = vec![
    //         Some(Fr::from(1u64)),
    //         Some(Fr::from(2u64)),
    //         Some(Fr::from(3u64)),
    //     ];

    //     // Evaluate the term at the given points
    //     let (result_scalar, result_term) = term.selected_evaluation(&points);

    //     // Expected result: (2^2 * 3^3, no remaining term)
    //     let expected_scalar = Fr::from(8u64) * Fr::from(27u64); // 2^2 * 3^3
    //     let expected_term = SparseTerm::new(vec![]); // No remaining term

    //     assert_eq!(result_scalar, expected_scalar);
    //     assert_eq!(result_term, expected_term);
    // }

    #[test]
    fn test_selected_evaluation_partial_points() {
        // Example SparseTerm: x_1^2 * x_2^3
        let term = SparseTerm::new(vec![(1, 2), (2, 3)]);
        let points: Vec<Option<Fr>> = vec![Some(Fr::from(1u64)), None, Some(Fr::from(3u64))];

        // Evaluate the term at the given points
        let (result_scalar, result_term) = term.selected_evaluation(&points);

        // Expected result: (1^2 * 3^3, x_2^3)
        let expected_scalar = Fr::from(1u64) * Fr::from(27u64); // 1^2 * 3^3
        let expected_term = SparseTerm::new(vec![(1, 2)]); // Remaining term with variable 1

        assert_eq!(result_scalar, expected_scalar);
        assert_eq!(result_term, expected_term);
    }

    #[test]
    fn test_selected_evaluation_no_points() {
        // Example SparseTerm: x_1^2 * x_2^3
        let term = SparseTerm::new(vec![(1, 2), (2, 3)]);
        let points: Vec<Option<Fr>> = vec![None, None, None];

        // Evaluate the term at the given points
        let (result_scalar, result_term) = term.selected_evaluation(&points);

        // Expected result: (1, x_1^2 * x_2^3)
        let expected_scalar = Fr::one(); // No points were evaluated, so scalar is 1
        let expected_term = SparseTerm::new(vec![(1, 2), (2, 3)]); // The full term remains

        assert_eq!(result_scalar, expected_scalar);
        assert_eq!(result_term, expected_term);
    }
}
