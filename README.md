# OffsetCalculator
A small tool that is meant to be the easiest way to calculate with (unsigned) hexadecimal values. Mostly used when, idk, trying to figure out what it says in a raw binary file, and you have these offsets in hexadecimal and want to compute between them.

## Compilation
The project is managed using Rust's Cargo, so download that first (the easiest way is with [rustup](https://rustup.io)).

Once you have that, navigate to the project's source directory and build the package and dependencies with:
```
cargo build --release
```

## Running
Once compiled, a binary can be found under `target/release/offsetcalculator`. This binary takes a few command line arguments:
 * **TBD**

When run normally, the binary enters a REPL (a _read-eval-print loop_), where expressions and statements can be written on each line. After such a line, the calculator evaluates it and prints the result.  
**TBD**

## Issues
**TBD**

## License
**TBD**
