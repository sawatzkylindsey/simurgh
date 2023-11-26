use blarg::{CommandLineParser, prelude::*, Parameter, Scalar};
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use std::cmp;
use std::convert::TryInto;
use std::str::FromStr;

const PLAYER_A: &str = "Player A";
const PLAYER_B: &str = "Player B";

#[derive(Debug)]
enum Gameplay {
    PlayerPlayer,
    PlayerComputer,
    ComputerPlayer,
    ComputerComputer,
}

impl std::fmt::Display for Gameplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gameplay::PlayerPlayer => write!(f, "pp | PlayerPlayer"),
            Gameplay::PlayerComputer => write!(f, "pc | PlayerComputer"),
            Gameplay::ComputerPlayer => write!(f, "cp | ComputerPlayer"),
            Gameplay::ComputerComputer => write!(f, "cc | ComputerComputer"),
        }
    }
}

impl FromStr for Gameplay {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "pp" | "playerplayer" => Ok(Gameplay::PlayerPlayer),
            "pc" | "playercomputer" => Ok(Gameplay::PlayerComputer),
            "cp" | "computerplayer" => Ok(Gameplay::ComputerPlayer),
            "cc" | "computercomputer" => Ok(Gameplay::ComputerComputer),
            _ => Err(format!("unknown: {}", value)),
        }
    }
}

fn main() {
    let mut rounds: usize = 0;
    let mut gameplay: Gameplay = Gameplay::ComputerComputer;

    let clp = CommandLineParser::new("rock-paper-scissors");
    let parser = clp
        .add(
            Parameter::argument(Scalar::new(&mut rounds), "rounds")
                .help("The number of rounds to play."),
        )
        .add(
            Parameter::argument(Scalar::new(&mut gameplay), "gameplay")
                .help("The type of gameplay.")
                .choice(Gameplay::PlayerPlayer, "A player vs. player game.")
                .choice(Gameplay::PlayerComputer, "A player vs. computer game.")
                .choice(Gameplay::ComputerPlayer, "A computer vs. player game.")
                .choice(Gameplay::ComputerComputer, "A computer vs. computer game."),
        )
        .build();
    parser.parse();

    let score: (usize, usize, usize) = (0..rounds)
        .map(|round| {
            println!("Round {}", round + 1);
            generate_round(&gameplay)
        })
        .map(|(a, b)| {
            let mut c_a = 0;
            let mut c_b = 0;
            let mut c_t = 0;

            let winner = match (a > b, b > a) {
                (true, false) => {
                    c_a += 1;
                    Some(PLAYER_A)
                }
                (false, true) => {
                    c_b += 1;
                    Some(PLAYER_B)
                }
                (_, _) => {
                    c_t += 1;
                    None
                }
            };

            match winner {
                Some(player) => println!("{} vs {} -> {} won!", a, b, player),
                None => println!("{} vs {} -> Tie!", a, b),
            };

            (c_a, c_b, c_t)
        })
        .fold((0, 0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1, acc.2 + i.2));

    println!("{} won {} rounds.", PLAYER_A, score.0);
    println!("{} won {} rounds.", PLAYER_B, score.1);
    println!("They tied {} rounds.", score.2);
}

fn generate_round(gameplay: &Gameplay) -> (Hand, Hand) {
    match gameplay {
        Gameplay::PlayerPlayer => (human_hand(PLAYER_A), human_hand(PLAYER_B)),
        Gameplay::PlayerComputer => (human_hand(PLAYER_A), computer_hand()),
        Gameplay::ComputerPlayer => (computer_hand(), human_hand(PLAYER_B)),
        Gameplay::ComputerComputer => (computer_hand(), computer_hand()),
    }
}

fn computer_hand() -> Hand {
    thread_rng().gen()
}

fn user_input(prompt: String) -> String {
    let mut value = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut value).unwrap();
    value
}

fn human_hand(player: &str) -> Hand {
    let mut result = user_input(format!("{}, go (rock, paper, scissor):", player)).try_into();

    loop {
        match result {
            Ok(hand) => return hand,
            Err(error) => {
                println!("{}", error);
                result = user_input(format!("Try again for {}:", player)).try_into();
            }
        }
    }
}

const HANDS: [Hand; 3] = [Hand::Rock, Hand::Paper, Hand::Scissor];

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::convert::TryFrom<String> for Hand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "rock" => Ok(Hand::Rock),
            "paper" => Ok(Hand::Paper),
            "scissor" => Ok(Hand::Scissor),
            _ => Err(format!("Invalid hand '{}'", value.trim())),
        }
    }
}

impl cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use Hand::*;
        match (&self, other) {
            (Rock, Paper) => cmp::Ordering::Less,
            (Rock, Scissor) => cmp::Ordering::Greater,
            (Paper, Rock) => cmp::Ordering::Greater,
            (Paper, Scissor) => cmp::Ordering::Less,
            (Scissor, Paper) => cmp::Ordering::Greater,
            (Scissor, Rock) => cmp::Ordering::Less,
            (_, _) => cmp::Ordering::Equal,
        }
    }
}

impl Distribution<Hand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hand {
        HANDS[rng.gen_range(0..3)]
    }
}
