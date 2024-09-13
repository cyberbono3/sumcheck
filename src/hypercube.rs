use ark_ff::Field;

#[derive(Debug, Clone)]
pub struct HyperCube<FF> {
    current: Vec<FF>,
    ended: bool,
    max_value: FF,
}

impl<FF: Field> HyperCube<FF> {
    pub fn new(num_vars: usize, max_value: FF) -> Self {
        Self {
            current: vec![FF::zero(); num_vars],
            ended: false,
            max_value,
        }
    }
}

impl<FF: Field> Iterator for HyperCube<FF> {
    type Item = Vec<FF>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        // get the result to return
        let result = self.current.clone();

        // pre-compute next result
        let mut incremented_a_var = false;

        self.current
            .iter_mut()
            .filter_map(|var| {
                if *var < self.max_value {
                    *var += FF::one();
                    incremented_a_var = true;
                    Some(())
                } else {
                    *var = FF::zero();
                    None
                }
            })
            .next();

        if !incremented_a_var {
            self.ended = true;
            self.current = vec![];
        }

        // return result
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{Zero, One};
    use ark_bn254::Fr; // Use Fr from the ark_bn254 crate, which is a predefined finite field.

    #[test]
    fn test_hypercube_initialization() {
        let max_value = Fr::from(5u64);
        let hypercube = HyperCube::new(3, max_value);

        assert_eq!(hypercube.current, vec![Fr::zero(); 3]);
        assert!(!hypercube.ended);
        assert_eq!(hypercube.max_value, max_value);
    }

    #[test]
    fn test_hypercube_iteration() {
        let max_value = Fr::from(2u64);
        let mut hypercube = HyperCube::new(2, max_value);

        // First step should be [0, 0]
        assert_eq!(hypercube.next(), Some(vec![Fr::zero(), Fr::zero()]));

        // Next should be [1, 0]
        assert_eq!(hypercube.next(), Some(vec![Fr::one(), Fr::zero()]));

        // Next should be [2, 0]
        assert_eq!(hypercube.next(), Some(vec![Fr::from(2u64), Fr::zero()]));

        // Next should be [0, 1]
        assert_eq!(hypercube.next(), Some(vec![Fr::zero(), Fr::one()]));

        // Next should be [1, 1]
        assert_eq!(hypercube.next(), Some(vec![Fr::one(), Fr::one()]));

        // Continue this pattern...
        assert_eq!(hypercube.next(), Some(vec![Fr::from(2u64), Fr::one()]));
        assert_eq!(hypercube.next(), Some(vec![Fr::zero(), Fr::from(2u64)]));
        assert_eq!(hypercube.next(), Some(vec![Fr::one(), Fr::from(2u64)]));
        assert_eq!(hypercube.next(), Some(vec![Fr::from(2u64), Fr::from(2u64)]));

        // After reaching the max values, it should return None
        assert_eq!(hypercube.next(), None);
    }

    #[test]
    fn test_hypercube_completed_iteration() {
        let max_value = Fr::from(1u64);
        let mut hypercube = HyperCube::new(1, max_value);

        // Expecting two iterations [0], [1], then None
        assert_eq!(hypercube.next(), Some(vec![Fr::zero()]));
        assert_eq!(hypercube.next(), Some(vec![Fr::one()]));
        assert_eq!(hypercube.next(), None);
    }
}