pub trait UserInterface {
    fn main_menu(&mut self) -> MainMenu;
    fn display_first_dealer(&mut self, first_dealer: bool);
}

pub enum MainMenu {
    Play,
    Exit,
}

