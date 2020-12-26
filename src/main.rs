// traits
mod ui;
mod opponent;

// structs
mod client;
mod termion_ui;
mod bogus_opponent;
mod game;
mod card;
mod hand;

fn main() {
    let ui = termion_ui::UI{};

    let c = client::Client::new(ui);
    c.run();
}

