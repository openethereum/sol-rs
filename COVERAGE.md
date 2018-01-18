# Generating Rust Code Coverage for Coveralls

Install `cargo-cov`, and make sure `llvm-cov` is on your path:

```
cargo install cargo-cov
which llvm-cov
```

Run cargo-cov with nightly rust in the crate needing coverage:

```
cd /path/to/crate-root
rustup run nightly cargo cov test [-p package-name] [--bin binary-name] # for testing specific packages / binaries
```

Cargo-cov will instrument the crate's code with rustc's `-Zprofiling` flag, enabling coverage analysis.
There must be NO panics in the crate being analyzed, or its tests. This may change in the future.

After a successful run of the crate's tests, `cargo-cov` outputs `.gcda` / `.gcno` files to `crate-root/target/cov/build/{gcda,gcno}`.

If you only want a nice chart of the coverage statistics, run `rustup run nightly cargo cov report [--open]`, to generate a nice HTML readout. 

## Getting .gcov files from gcda / gcno sources for coveralls

This part is a little trickier. There is a tool by @marco-c called `grcov` which ingests gcda / gcno files, and outputs coveralls (and other) formatted files.
At the time of writing, the tool runs into issues with gcda / gcno files generated using both llvm and gcc code.

It is still possible to get `.gcov` files using `llvm-cov gcov`, which falls back to a format compatible with GCC 4.2 (gcov format breaking changes after 4.2).

Cargo-cov's output files come with a leading hash to make merging work across multiple runs. 
So, the first thing to do is remove these leading hashes, and place all the gcda / gcno files in a single directory.

Another sticking point, `llvm-cov gcov` assumes that the gcda / gcno files share the same name as the instrumented source file (hangover from C/C++).
That convention is fine, since `llvm-cov` allows one to specify gcda / gcno files manually.

As an example:
```
cd path/to/crate-root
mkdir gcov
cd gcov
llvm-cov gcov -b -gcda=../target/cov/build/gcda/analyzed-source-file.gcda -gcno=../target/cov/build/gcno/analyzed-source-file.gcno ../path/to/analyzed-source-file.rs
```

This will generate a number of `.gcov` files in the current working directory, complete with line and branching statistics (w/ the `-b` flag)!

There appear to be a couple tools avalailable for uploading `.gcov` to coveralls using their JSON API.

