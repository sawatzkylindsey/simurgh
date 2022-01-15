
use argparse::{ArgumentParser, Store};

use simurgh_core::player::PlayerType;

fn main() {
    let mut player_1: PlayerType = PlayerType::Automatic;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut player_1)
            .add_argument("player-1", Store, "The type[-name] of player one (ex: Manual-Alice, Automatic, etc).");
        ap.parse_args_or_exit();
    }

    println!("{}", player_1.to_string());
}

