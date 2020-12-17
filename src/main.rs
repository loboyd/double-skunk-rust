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
    let ui = termion_ui::UI{};

    let c = client::Client::new(ui);
    c.run();
}

