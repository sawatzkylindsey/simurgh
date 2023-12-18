use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use pprof::flamegraph::Options;
use std::str::FromStr;
use sudoku::model::board::SudokuProblem;
use sudoku::solver::search::Search;

const EASY_SUITE: [&str; 8] = [
    "016400000200009000400000062070230100100000003003087040960000005000800007000006820",
    "049008605003007000000000030000400800060815020001009000010000000000600400804500390",
    "760500000000060008000000403200400800080000030005001007809000000600010000000003041",
    "000605000003020800045090270500000001062000540400000007098060450006040700000203000",
    "409000705000010000006207800200000009003704200800000004002801500000060000905000406",
    "000010030040070501002008006680000003000302000300000045200500800801040020090020000",
    "080070030260050018000000400000602000390010086000709000004000800810040052050090070",
    "000093006000800900020006100000080053006000200370050000002500040001009000700130000",
];
const HARD_SUITE: [&str; 2] = [
    "800000000003600000070090200050007000000045700000100030001000068008500010090000400",
    "004070000800000401013004000000000000120000804000009700600007002000100060000308970",
];

pub fn dfs_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("depth-first-search");
    let mut counter = 0;

    group.bench_function("easy-suite", move |b| {
        b.iter_batched(
            || {
                let sdm = EASY_SUITE[counter];
                counter = (counter + 1) % EASY_SUITE.len();
                Search::new(SudokuProblem::from_str(sdm).unwrap())
            },
            |search| {
                search.run().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    counter = 0;
    group.bench_function("hard-suite", move |b| {
        b.iter_batched(
            || {
                let sdm = HARD_SUITE[counter];
                counter = (counter + 1) % HARD_SUITE.len();
                let p = SudokuProblem::from_str(sdm).unwrap();
                Search::new(p)
            },
            |search| {
                search.run().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).with_profiler(PProfProfiler::new(
        100,
        Output::Flamegraph(Some(Options::default()))
    ));
    targets = dfs_benchmarks
);
criterion_main!(benches);
