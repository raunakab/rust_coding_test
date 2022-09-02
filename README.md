# Rust Coding Challenge
## Description
This toy-engine implements a streamed version in which the data is streamed through memory instead of being directly loaded *all* into memory and then operated on.

Furthermore, the engine separated into 3 phases:
1. Deserialization
    - This phase reads the next line in the file into a buffer (overwriting the previous record).
    - A `callback: F` where `F: FnMut(Transaction)` is passed in as an argument which is called with the deserialized record upon each successful read.
2. Core-Engine
    - The core engine is the actual engine which interprets each transaction and tracks the state of clients.
    - Inside of the `callback` function, `core.process` function is called with the deserialized `Transaction` struct.
3. Serialization
    - Once the iteration through the file has ceased, the serialization phase starts by retrieving all the clients from the `core` engine. It then writes these records to `stdout`.

## Scoring Criteria
### Basics
The application fully builds with:
```bash
cargo build
cargo run -- transactions.csv > accounts.csv
```
It reads in the data as according to the requested command-line prompt and outputs the data in the requested form.
The entire crate is formatted using `cargo +nightly fmt`, and the style configurations are located in `rustfmt.toml`.

### Completeness
All transaction types have been completed.
The software is able to handle `deposits`, `withdrawals`, `disputes`, `resolves`, and `chargebacks`.

### Correctness
I have written unit tests for the `deserialization` and `core` phases of the engine.
The serialization phase was tested by actually running the binary.
The entire engine as a whole was also tested in this similar fashion.
The example datasets I ran the engine against can be found in the [`./assets`](assets) directory.

All the custom tests are passing.
I also believe that I have covered interesting edge cases (i.e., disputing one transaction multiple times, resolving a non-disputed transaction, etc.).

As a minor digression, `rust` is a **wonderful** language in that it forces developers to exhaustively match on all cases (or a compile-time error will be thrown).
This forces vigilance, an understandably crucial trait in the crypto-space.

### Safety and Robustness
I am using **no** unsafe features/functions.
This includes:
1. No unsafe blocks of code, i.e.,:
```rust
unsafe {
    // unsafe code here!
    // ...
}
```
2. No `panic!()`'s, `Result::unwrap`'s, `Result::expect`'s.
Instead, functions performing fallible operations return a `Result` type.
Errors are propagated using the shortcircuiting `?` operator.

### Efficiency
The reading in phase has been optimized to perform **ammortized allocation**.
Instead of allocating a new space for each row of the inputted csv, a (growable if necessary) buffer is allocated into which each new record is mutably read into (by overwriting the previous entry).

This is more optimal as compared to reading the entire file into memory and then operating on each record, since that implementation would use `O(n)` space, where `n` is the number of rows in the csv file.

### Maintainability
The engine has been separated into 3, respective phases, each of which outputs the input to the next.
This is, in my opinion, the optimal design because any phase can be completely switched out with another **without** requiring the other phases to be switched out as well.

For example, if there was an optimization opportunity with the `core` engine (i.e., the one that does all the processing), it could be switched out with another module which exports the same function with the same signature.
This would require no change to the deserialization and serialization phases.

Furthermore, extending the functionality to include another transaction type would be simple as adding another entry to the `Transaction` enum, adding the appropriate method to the `Client` to handle this new type of transaction, and calling this method from the `core` engine upon pattern-match.
