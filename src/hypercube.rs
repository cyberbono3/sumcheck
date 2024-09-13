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
