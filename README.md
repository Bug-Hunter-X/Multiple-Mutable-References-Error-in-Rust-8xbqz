# Multiple Mutable References in Rust
This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same data.  Rust's borrow checker prevents this to guarantee memory safety and avoid data races. The example shows the error and how to refactor the code to fix it.