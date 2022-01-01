
use std::cmp;
use std::convert::TryInto;
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Standard};
use argparse::{ArgumentParser, Store};


const PLAYER_A: &str = "Player A";
const PLAYER_B: &str = "Player B";


fn main() {
    let mut rounds: usize = 0;
    let mut gameplay: String = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut rounds)
            .add_argument("rounds", Store, "the number of rounds to play");
        ap.refer(&mut gameplay)
            .add_argument("gameplay", Store, "the type of gameplay -- wish it could be a 'choice'");
        ap.parse_args_or_exit();
    }

    let score: (usize, usize, usize) = (0..rounds)
        .map(|round| {
            println!("Round {}", round + 1);
            generate_round(&gameplay)
        })
        .map(|(a, b)| {
            let mut cA = 0;
            let mut cB = 0;
            let mut cT = 0;

            let winner = match (a > b, b > a) {
                (true, false) => {
                    cA += 1;
                    Some(PLAYER_A)
                },
                (false, true) => {
                    cB += 1;
                    Some(PLAYER_B)
                },
                (_, _) => {
                    cT += 1;
                    None
                },
            };

            match winner {
                Some(player) => println!("{} vs {} -> {} won!", a, b, player),
                None => println!("{} vs {} -> Tie!", a, b),
            };

            (cA, cB, cT)
        })
        .fold((0, 0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1, acc.2 + i.2));

    println!("{} won {} rounds.", PLAYER_A, score.0);
    println!("{} won {} rounds.", PLAYER_B, score.1);
    println!("They tied {} rounds.", score.2);
}


fn generate_round(gameplay: &String) -> (Hand, Hand) {
    match gameplay.as_str() {
        "pp" => (human_hand(PLAYER_A), human_hand(PLAYER_B)),
        "cp" => (computer_hand(), human_hand(PLAYER_B)),
        "pc" => (human_hand(PLAYER_A), computer_hand()),
        _ => (computer_hand(), computer_hand()),
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

