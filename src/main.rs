mod ui;
mod opponent;
mod stdin_ui;
mod bogus_opponent;

fn main() {
    let ui = stdin_ui::StdinInterface {
        height: 10, width: 10
    };

    run(&ui);
}

fn run<T: ui::UserInterface>(t: &T) {
    let opponent = bogus_opponent::BogusOpponent {};

    loop {
        // TODO: Learn how to handle errors here, e.g., resulting from a user
        // passing in an empty string
        match t.main_menu() {
            //1 => println!("somebody's looking for a good time"),
            1 => play_game(&opponent),
            2 => std::process::exit(0),
            _ => println!("great, now we're lost ... "),
        }
    }
}

fn play_game<T: opponent::Opponent>(opp: &T) {
    // TODO: Replace this with calls to the UI (how?)
    match opp.determine_first_dealer() {
        true => println!("You're the first dealer"),
        false => println!("You're opponent is the first dealer"),
    }

    loop {
        // TODO: Where should the `dealer` state exist?
        let (_hand, _starter) = opp.deal();

        // TODO: discard (get crib)

        // TODO: pegging phase

        // TODO: get opponents hand+crib

        // TODO: count hands

        // TODO: check game termination

        break;
    }
}

