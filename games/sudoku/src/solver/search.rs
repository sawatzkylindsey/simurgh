use crate::model::board::SudokuProblem;
use crate::model::cell::SudokuValue;

#[derive(Debug)]
pub struct Search {
    problem: SudokuProblem,
}

impl Search {
    pub fn new(problem: SudokuProblem) -> Search {
        Self { problem }
    }

    pub fn run(&self) -> Result<SudokuProblem, ()> {
        let mut stack = vec![self.problem.clone()];

        while !stack.is_empty() {
            let current = stack.pop().expect("stack must not be empty");

            match current.position(&SudokuValue::Unknown) {
                Some(cell) => {
                    for candidate in SudokuValue::candidates() {
                        let problem = current.replace(&cell, *candidate);

                        if problem.is_valid() {
                            stack.push(problem);
                        }
                    }
                }
                None => {
                    if current.is_complete() {
                        return Ok(current);
                    }
                }
            }
        }

        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn search() {
        // Setup
        let problem = SudokuProblem::from_str(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        )
        .unwrap();
        let search = Search::new(problem);

        // Execute
        let result = search.run().unwrap();

        // Verify
        assert_eq!(
            result,
            SudokuProblem::from_str(
                "379526814564318972281479365435267198698143527712895436923754681146982753857631249"
            )
            .unwrap()
        );
    }
}
