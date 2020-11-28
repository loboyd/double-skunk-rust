pub trait UserInterface {
    fn main_menu(&self) -> u8;
    fn first_dealer(&self, first_dealer: bool);
}

