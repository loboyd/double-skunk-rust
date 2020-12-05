pub trait UserInterface {
    fn main_menu(self) -> MainMenu;
    //fn display_first_dealer(self, first_dealer: bool);
}

pub enum MainMenu {
    Play,
    Exit,
}

