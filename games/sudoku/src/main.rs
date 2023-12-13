// use crate::model::board::SudokuProblem;
// use crate::solver::search::Search;
use blarg::{CommandLineParser, GeneralParser, Parameter, Scalar};
use sudoku::model::board::SudokuProblem;
use sudoku::solver::search::Search;

#[derive(Debug)]
struct Parameters {
    problem: SudokuProblem,
}

fn main() {
    let parameters = parse();
    println!("{parameters:?}");

    let solver = Search::new(parameters.problem);
    solver.run();
}

fn parse() -> Parameters {
    parse_tokens(|parser: GeneralParser| Ok(parser.parse()))
}

fn parse_tokens(parse_fn: impl FnOnce(GeneralParser) -> Result<(), i32>) -> Parameters {
    let mut parameters = Parameters {
        problem: SudokuProblem::default(),
    };

    let clp = CommandLineParser::new(env!("CARGO_BIN_NAME"));
    let parser = clp.add(Parameter::argument(Scalar::new(&mut parameters.problem), "problem")
        .help("The full sudoku puzzle in single line format (sdm).  This format interprets the cells from left to right, top to bottom.")
        .meta(vec!["ex: 379000014060010070080009005435007000090040020000800436900700080040080050850000249"])).build();
    // The parse_fn signature is a `Result`.
    // However, since `GeneralParser::parse` does not return an error (it uses `std::process::exit` under the hood), the `Err` case is only reached via test.
    parse_fn(parser).expect("test-reachable-only");
    parameters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_none() {
        // Setup
        let tokens = vec!["27206f43-9a49-4ff7-b0ef-b493acb683a5", "none"];

        // Execute & verify
        parse_tokens(|parser| parser.parse_tokens(tokens.as_slice()));
    }
}
