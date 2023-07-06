use rand::seq::SliceRandom;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decrement {
    None,
    Predecrement,
    Postincrement,
}
impl Decrement {
    pub fn get_random() -> Decrement {
        use Decrement::*;
        [None, Predecrement, Postincrement]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}