
### Development

    # Build.
    # generates `./target/release/sudoku`
    cargo build --workspace --release

    # Testing
    cargo test --workspace

    cd games/sudoku

    # Performance testing
    cargo bench --bench dfs
    ls -al ../../target/criterion/depth-first-search/

    # Generate a flamegraph
    ## Currentyl, this doesn't work: "SIGBUS: access to undefined memory"
    ## See: https://github.com/tikv/pprof-rs/issues/210
    cargo bench --bench dfs -- --profile-time 30
