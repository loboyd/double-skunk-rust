pub trait Opponent {
    fn determine_first_dealer(&self) -> bool;
    fn deal(&self) -> (Vec<u8>, u8);
}

