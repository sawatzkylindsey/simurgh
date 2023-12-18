use crate::model::cell::{Cell, SudokuValue};
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;
use std::str::FromStr;

lazy_static! {
    static ref PROBLEM_REGEX: Regex = Regex::new(r"^[0-9]{81}$").unwrap();
}

#[derive(Clone, PartialEq, Eq)]
pub struct SudokuProblem {
    puzzle: [SudokuValue; 9 * 9],
}

impl Default for SudokuProblem {
    fn default() -> Self {
        Self {
            puzzle: [SudokuValue::Unknown; 9 * 9],
        }
    }
}

impl FromStr for SudokuProblem {
    type Err = ();

    fn from_str(sdm: &str) -> Result<Self, Self::Err> {
        let puzzle: Result<[SudokuValue; 9 * 9], _> = sdm
            .chars()
            .map(SudokuValue::try_from)
            .collect::<Result<Vec<SudokuValue>, ()>>()?
            .try_into();
        match puzzle {
            Ok(puzzle) => Ok(SudokuProblem::new(puzzle)),
            Err(_) => Err(()),
        }
    }
}

impl std::fmt::Debug for SudokuProblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.puzzle.iter().map(|v| v.to_char()).collect::<String>()
        )
    }
}

impl SudokuProblem {
    fn new(puzzle: [SudokuValue; 9 * 9]) -> Self {
        Self { puzzle }
    }

    fn rows(&self) -> Vec<[SudokuValue; 9]> {
        let mut rows: Vec<[SudokuValue; 9]> = Vec::with_capacity(9);

        for i in 0..9 {
            let row = &self.puzzle[i * 9..(i * 9) + 9];
            rows.push(
                row.try_into()
                    .expect("slice must fit into [SudokuValue; 9]"),
            );
        }

        rows
    }

    fn columns(&self) -> Vec<[SudokuValue; 9]> {
        let mut columns: Vec<[SudokuValue; 9]> = Vec::with_capacity(9);

        for j in 0..9 {
            let mut column: [SudokuValue; 9] = [SudokuValue::Unknown; 9];

            for i in 0..9 {
                let index = (&i * 9) + &j;
                column[i] = self.puzzle[index];
            }

            columns.push(column);
        }

        columns
    }

    fn squares(&self) -> Vec<[SudokuValue; 9]> {
        let mut squares: Vec<[SudokuValue; 9]> = Vec::with_capacity(9);

        for x in 0..9 {
            let mut square: [SudokuValue; 9] = [SudokuValue::Unknown; 9];

            let a = &x / 3;
            let b = &x % 3;
            let initial = (a * 9 * 3) + (b * 3);

            for y in 0..9 {
                square[y] = self.puzzle[&initial + ((&y / 3) * 9) + (&y % 3)];
            }

            squares.push(square);
        }

        squares
    }

    pub fn is_valid(&self) -> bool {
        let valid_check = |section: &[SudokuValue]| {
            let mut counts: [usize; 9] = [0; 9];

            for value in section {
                if let Some(index) = value.to_count_index() {
                    counts[*&index] += 1;

                    if counts[*&index] > 1 {
                        return false;
                    }
                }
            }

            true
        };

        self.rows().iter().all(|section| valid_check(&section[..]))
            && self
                .columns()
                .iter()
                .all(|section| valid_check(&section[..]))
            && self
                .squares()
                .iter()
                .all(|section| valid_check(&section[..]))
    }

    pub fn is_complete(&self) -> bool {
        let solved_check = |section: &[SudokuValue]| {
            for value in section {
                if let None = value.to_count_index() {
                    return false;
                }
            }

            true
        };

        self.rows().iter().all(|section| solved_check(&section[..]))
            && self
                .columns()
                .iter()
                .all(|section| solved_check(&section[..]))
            && self
                .squares()
                .iter()
                .all(|section| solved_check(&section[..]))
    }

    pub(crate) fn replace(&self, cell: &Cell, value: SudokuValue) -> SudokuProblem {
        let mut puzzle = self.puzzle.clone();
        puzzle[cell.index] = value;
        SudokuProblem::new(puzzle)
    }

    pub fn position(&self, value: &SudokuValue) -> Option<Cell> {
        self.puzzle
            .iter()
            .position(|v| v == value)
            .map(|i| Cell::index(i).expect("must be a valid index"))
    }
}

impl SudokuValue {
    fn to_count_index(&self) -> Option<usize> {
        match self {
            SudokuValue::Unknown => None,
            SudokuValue::One => Some(0),
            SudokuValue::Two => Some(1),
            SudokuValue::Three => Some(2),
            SudokuValue::Four => Some(3),
            SudokuValue::Five => Some(4),
            SudokuValue::Six => Some(5),
            SudokuValue::Seven => Some(6),
            SudokuValue::Eight => Some(7),
            SudokuValue::Nine => Some(8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections() {
        /*
        136524798
        895367124
        724981356
        583649271
        261873945
        479152683
        642718539
        957436812
        318295467
         */
        let problem = SudokuProblem::from_str(
            "136524798895367124724981356583649271261873945479152683642718539957436812318295467",
        )
        .unwrap();
        assert_eq!(
            problem.rows(),
            vec![
                // Rows
                as_section("136524798"),
                as_section("895367124"),
                as_section("724981356"),
                as_section("583649271"),
                as_section("261873945"),
                as_section("479152683"),
                as_section("642718539"),
                as_section("957436812"),
                as_section("318295467"),
            ]
        );
        assert_eq!(
            problem.columns(),
            vec![
                // Rows
                as_section("187524693"),
                as_section("392867451"),
                as_section("654319278"),
                as_section("539681742"),
                as_section("268475139"),
                as_section("471932865"),
                as_section("713296584"),
                as_section("925748316"),
                as_section("846153927"),
            ]
        );
        assert_eq!(
            problem.squares(),
            vec![
                // Rows
                as_section("136895724"),
                as_section("524367981"),
                as_section("798124356"),
                as_section("583261479"),
                as_section("649873152"),
                as_section("271945683"),
                as_section("642957318"),
                as_section("718436295"),
                as_section("539812467"),
            ]
        );
    }

    #[test]
    fn is_valid() {
        /*
        379000014
        060010070
        080009005
        435007000
        090040020
        000800436
        900700080
        040080050
        850000249
         */
        let problem = SudokuProblem::from_str(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        )
        .unwrap();
        assert!(problem.is_valid());

        // Row invalid
        let problem = SudokuProblem::from_str(
            "379300014060010070080009005435007000090040020000800436900700080040080050850000249",
        )
        .unwrap();
        assert!(!problem.is_valid());

        // Column invalid
        let problem = SudokuProblem::from_str(
            "379000014360010070080009005435007000090040020000800436900700080040080050850000249",
        )
        .unwrap();
        assert!(!problem.is_valid());

        // Square invalid
        let problem = SudokuProblem::from_str(
            "379000014063010070080009005435007000090040020000800436900700080040080050850000249",
        )
        .unwrap();
        assert!(!problem.is_valid());
    }

    #[test]
    fn is_complete() {
        /*
        136524798
        895367124
        724981356
        583649271
        261873945
        479152683
        642718539
        957436812
        318295467
         */
        let problem = SudokuProblem::from_str(
            "136524798895367124724981356583649271261873945479152683642718539957436812318295467",
        )
        .unwrap();
        assert!(problem.is_complete());

        // Row incomplete
        let problem = SudokuProblem::from_str(
            "136524790895367124724981356583649271261873945479152683642718539957436812318295467",
        )
        .unwrap();
        assert!(!problem.is_complete());

        // Column incomplete
        let problem = SudokuProblem::from_str(
            "136524798895367124724981356583649271261873945479152683642718539957436812018295467",
        )
        .unwrap();
        assert!(!problem.is_complete());

        // Square incomplete
        let problem = SudokuProblem::from_str(
            "136524798895367124720981356583649271261873945479152683642718539957436812318295467",
        )
        .unwrap();
        assert!(!problem.is_complete());
    }

    #[test]
    fn replace() {
        let problem = SudokuProblem::from_str(
            "012345678901234567890123456789012345678901234567890123456789012345678901234567890",
        )
        .unwrap();

        assert_eq!(
            problem.replace(&Cell::row_column(0, 0).unwrap(), SudokuValue::One),
            SudokuProblem::from_str(
                "112345678901234567890123456789012345678901234567890123456789012345678901234567890",
            )
            .unwrap()
        );
        assert_eq!(
            problem.replace(&Cell::row_column(0, 8).unwrap(), SudokuValue::Nine),
            SudokuProblem::from_str(
                "012345679901234567890123456789012345678901234567890123456789012345678901234567890",
            )
            .unwrap()
        );
    }

    #[test]
    fn position() {
        let problem = SudokuProblem::from_str(
            "012345678901234567890123456789012345678901234567890123456789012345678901234567890",
        )
        .unwrap();

        assert_eq!(
            problem.position(&SudokuValue::Unknown),
            Some(Cell::index(0).unwrap()),
        );

        assert_eq!(
            problem.position(&SudokuValue::One),
            Some(Cell::index(1).unwrap()),
        );

        assert_eq!(SudokuProblem::default().position(&SudokuValue::One), None,);
    }

    fn as_section(snippet: &str) -> [SudokuValue; 9] {
        let tmp = snippet
            .chars()
            .map(|c| SudokuValue::try_from(c).unwrap())
            .collect::<Vec<_>>();
        tmp.try_into().unwrap()
    }
}
