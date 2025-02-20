# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: modifying a vector through a raw pointer after the vector's capacity might have changed.  The raw pointer becomes invalid if the vector reallocates its internal memory. This example highlights the importance of safe memory management practices in Rust.

## How to reproduce

1. Clone this repository.
2. Run `cargo run`.
3. Observe the unpredictable output, demonstrating undefined behavior.