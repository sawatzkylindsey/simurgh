use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::str::FromStr;
use sudoku::model::board::SudokuProblem;

const PROBLEM_SUITE: [&str; 8] = [
    "016400000200009000400000062070230100100000003003087040960000005000800007000006820",
    "049008605003007000000000030000400800060815020001009000010000000000600400804500390",
    "760500000000060008000000403200400800080000030005001007809000000600010000000003041",
    "000605000003020800045090270500000001062000540400000007098060450006040700000203000",
    "409000705000010000006207800200000009003704200800000004002801500000060000905000406",
    "000010030040070501002008006680000003000302000300000045200500800801040020090020000",
    "080070030260050018000000400000602000390010086000709000004000800810040052050090070",
    "000093006000800900020006100000080053006000200370050000002500040001009000700130000",
];

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("problem suite", move |b| {
        b.iter_batched(
            || {
                PROBLEM_SUITE
                    .iter()
                    .map(|problem| SudokuProblem::from_str(problem).unwrap())
                    .collect::<Vec<SudokuProblem>>()
            },
            |mut problems| {
                problems.iter().for_each(|problem| {
                    problem.is_valid();
                });
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
