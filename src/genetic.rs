pub trait Individual {
    fn mutate(&mut self);
    fn cross(&self, other: &Self) -> Self;
}


pub struct State {
    chromosome: Vec<bool>,
}


pub fn genetic(initial_population: &[State], generations: usize) {
  
}
