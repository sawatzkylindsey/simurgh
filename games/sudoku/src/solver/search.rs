use crate::model::board::SudokuProblem;

pub struct Search {
    problem: SudokuProblem,
}

impl Search {
    pub fn new(problem: SudokuProblem) -> Search {
        Self { problem }
    }

    pub fn run(&self) {
        todo!()
    }
}
