# Rust Learning Lab

A small terminal app for exploring Rust by running focused examples. Each lesson
prints a result, then points out the language feature behind it.

## Run it

Install Rust with [rustup](https://rustup.rs/), then from this directory run:

```sh
cargo run
```

Run the included checks with:

```sh
cargo test
```

Enter a lesson number from the menu; its output stays visible until you press
Enter to return. Choose `8` to see the challenges or `q` to quit.

## What it demonstrates

- Variables, mutability, and shadowing
- `if`, `for`, arrays, and basic expressions
- Ownership, shared borrowing, and mutable borrowing
- Structs, enums, methods, and exhaustive `match`
- `Vec`, `HashMap`, and iterator adapters
- `Result` and explicit error handling
- Traits and functions that accept different implementations
- Unit tests

## Your challenges

1. **Priorities:** Add `Priority` (`Low`, `Normal`, `High`) to `Task`. Include it
	in the description and handle every variant with `match`.
2. **Word counter:** Implement `count_words(text: &str) -> HashMap<String, usize>`.
	Normalize words to lowercase before counting them.
3. **Task completion:** Add `Task::complete(&mut self) -> Result<(), String>`.
	Allow unfinished tasks to become `Done`, and return an error if they are
	already done. Write tests for both outcomes.
4. **Interactive analyzer:** Add a menu option that reads a line from the user
	and displays the word counts. Propagate I/O errors rather than panicking.

Try to solve each challenge with the Rust feature it is intended to practice:
enums and exhaustive matching, borrowed input and owned map keys, mutable
borrowing and `Result`, then standard input and error propagation.
