use std::convert::TryFrom;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Cell {
    row: usize,
    column: usize,
    pub(super) index: usize,
}

impl Cell {
    pub fn row_column(row: usize, column: usize) -> Result<Cell, ()> {
        if row < 9 && column < 9 {
            let index = (&row * 9) + &column;
            Ok(Self { row, column, index })
        } else {
            Err(())
        }
    }

    pub fn index(index: usize) -> Result<Cell, ()> {
        if index < 9 * 9 {
            let row = &index / 9;
            let column = &index % 9;
            Ok(Self { row, column, index })
        } else {
            Err(())
        }
    }
}

const CANDIDATES: [SudokuValue; 9] = [
    SudokuValue::One,
    SudokuValue::Two,
    SudokuValue::Three,
    SudokuValue::Four,
    SudokuValue::Five,
    SudokuValue::Six,
    SudokuValue::Seven,
    SudokuValue::Eight,
    SudokuValue::Nine,
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SudokuValue {
    Unknown,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl SudokuValue {
    fn value(&self) -> usize {
        match self {
            SudokuValue::Unknown => 0,
            SudokuValue::One => 1,
            SudokuValue::Two => 2,
            SudokuValue::Three => 3,
            SudokuValue::Four => 4,
            SudokuValue::Five => 5,
            SudokuValue::Six => 6,
            SudokuValue::Seven => 7,
            SudokuValue::Eight => 8,
            SudokuValue::Nine => 9,
        }
    }

    pub(crate) fn candidates() -> &'static [SudokuValue] {
        &CANDIDATES[..]
    }
}

impl TryFrom<char> for SudokuValue {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(SudokuValue::Unknown),
            '1' => Ok(SudokuValue::One),
            '2' => Ok(SudokuValue::Two),
            '3' => Ok(SudokuValue::Three),
            '4' => Ok(SudokuValue::Four),
            '5' => Ok(SudokuValue::Five),
            '6' => Ok(SudokuValue::Six),
            '7' => Ok(SudokuValue::Seven),
            '8' => Ok(SudokuValue::Eight),
            '9' => Ok(SudokuValue::Nine),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for SudokuValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl SudokuValue {
    pub(crate) fn to_char(&self) -> char {
        match self {
            SudokuValue::Unknown => '0',
            SudokuValue::One => '1',
            SudokuValue::Two => '2',
            SudokuValue::Three => '3',
            SudokuValue::Four => '4',
            SudokuValue::Five => '5',
            SudokuValue::Six => '6',
            SudokuValue::Seven => '7',
            SudokuValue::Eight => '8',
            SudokuValue::Nine => '9',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_cell() {
        assert_eq!(Cell::row_column(0, 0), Cell::index(0));
        assert_eq!(Cell::row_column(0, 1), Cell::index(1));
        assert_eq!(Cell::row_column(0, 2), Cell::index(2));
        assert_eq!(Cell::row_column(0, 3), Cell::index(3));
        assert_eq!(Cell::row_column(0, 4), Cell::index(4));
        assert_eq!(Cell::row_column(0, 5), Cell::index(5));
        assert_eq!(Cell::row_column(0, 6), Cell::index(6));
        assert_eq!(Cell::row_column(0, 7), Cell::index(7));
        assert_eq!(Cell::row_column(0, 8), Cell::index(8));

        assert_eq!(Cell::row_column(1, 0), Cell::index(9));
        assert_eq!(Cell::row_column(1, 1), Cell::index(10));
        assert_eq!(Cell::row_column(1, 8), Cell::index(17));

        assert_eq!(Cell::row_column(2, 0), Cell::index(18));
        assert_eq!(Cell::row_column(2, 1), Cell::index(19));
        assert_eq!(Cell::row_column(2, 8), Cell::index(26));

        assert_eq!(Cell::row_column(8, 8), Cell::index(80));
    }

    #[test]
    fn invalid_cell() {
        Cell::row_column(0, 9).unwrap_err();
        Cell::row_column(9, 0).unwrap_err();
        Cell::row_column(9, 9).unwrap_err();

        Cell::index(9 * 9).unwrap_err();
    }

    #[test]
    fn convert_value() {
        assert_eq!(
            SudokuValue::try_from(SudokuValue::Unknown.to_char()).unwrap(),
            SudokuValue::Unknown,
        );
        assert_eq!(SudokuValue::Unknown.value(), 0);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::One.to_char()).unwrap(),
            SudokuValue::One,
        );
        assert_eq!(SudokuValue::One.value(), 1);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Two.to_char()).unwrap(),
            SudokuValue::Two,
        );
        assert_eq!(SudokuValue::Two.value(), 2);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Three.to_char()).unwrap(),
            SudokuValue::Three,
        );
        assert_eq!(SudokuValue::Three.value(), 3);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Four.to_char()).unwrap(),
            SudokuValue::Four,
        );
        assert_eq!(SudokuValue::Four.value(), 4);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Five.to_char()).unwrap(),
            SudokuValue::Five,
        );
        assert_eq!(SudokuValue::Five.value(), 5);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Six.to_char()).unwrap(),
            SudokuValue::Six,
        );
        assert_eq!(SudokuValue::Six.value(), 6);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Seven.to_char()).unwrap(),
            SudokuValue::Seven,
        );
        assert_eq!(SudokuValue::Seven.value(), 7);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Eight.to_char()).unwrap(),
            SudokuValue::Eight,
        );
        assert_eq!(SudokuValue::Eight.value(), 8);

        assert_eq!(
            SudokuValue::try_from(SudokuValue::Nine.to_char()).unwrap(),
            SudokuValue::Nine,
        );
        assert_eq!(SudokuValue::Nine.value(), 9);
    }

    #[test]
    fn invalid_value() {
        SudokuValue::try_from('a').unwrap_err();
    }
}
