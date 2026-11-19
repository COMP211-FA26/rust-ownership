# Rust Ownership Examples 🦀

This repository contains Rust code examples created for my presentation on **Moving Ownership in Rust**.

## What's Included

- **Moving Ownership:** How assigning a `String` transfers ownership and invalidates the original variable.
- **Copying:** Why integers can be copied without losing access to the original.
- **Functions:** How passing a `String` into a function transfers ownership.
- **Borrowing and Cloning:** How to use a String without losing ownership.

## Running the Code

Make sure Rust and Cargo are installed.

```bash
cargo run --bin copy_example
cargo run --bin function_move_fixed
cargo run --bin borrowing_cloning
```

Some examples intentionally produce compiler errors to demonstrate Rust's ownership rules.
