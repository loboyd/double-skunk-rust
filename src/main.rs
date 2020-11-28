mod client;
mod ui;
mod opponent;
mod stdin_ui;
mod bogus_opponent;

fn main() {
    // set up user-interface
    let ui = stdin_ui::StdinInterface {
        height: 10, width: 10
    };

    // set up opponent
    let opponent = bogus_opponent::BogusOpponent{};

    let mut c = client::Client::new(opponent, ui);
    c.run();
}

