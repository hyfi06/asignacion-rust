pub trait Individual {
    fn mutate(&mut self);
    fn cross(&self, other: &Self) -> Self;
}

pub struct State {
    chromosome: Vec<bool>,
}

impl Individual for State {
    fn mutate(&mut self) {
        todo!()
    }

    fn cross(&self, other: &Self) -> Self {
        todo!()
    }
}

pub fn genetic(initial_population: &[State], generations: usize) {
  
}
