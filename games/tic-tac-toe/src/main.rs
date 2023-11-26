use blarg::{CommandLineParser, Parameter, Scalar};

use simurgh_core::player::PlayerType;

fn main() {
    let mut player_1: PlayerType = PlayerType::Automatic;

    let clp = CommandLineParser::new("tic-tac-toe");
    let parser = clp
        .add(
            Parameter::argument(Scalar::new(&mut player_1), "player-1")
                .help("The type[-name] of player one (ex: Manual-Alice, Automatic, etc)."),
        )
        .build();
    parser.parse();

    println!("{}", player_1.to_string());
}
