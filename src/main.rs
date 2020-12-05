// traits
mod ui;
mod opponent;

// structs
mod client;
//mod stdin_ui;
mod termion_ui;
mod bogus_opponent;
mod game;
mod card;

fn main() {
    // set up user-interface
    let ui = termion_ui::UI{};
    //let ui = stdin_ui::StdinInterface { height: 10, width: 10 };

    let mut c = client::Client::new(ui);
    c.run();
}

