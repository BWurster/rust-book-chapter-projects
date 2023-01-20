# Chapter 2: Programming a Guessing Game

This chapter is all about programming a first full Rust project that utilizes imported crates. It is all within a single file still though. Much of what is talked about is said to come up later in the book, but it is nonetheless good to jump right in.

Some big things that were done in this project were
- taking input
- utilizing `Result` enums
- using print placeholders
- random number generation
- comparisons

## Taking input

Taking input is done with the `io::stdin()` function from the `std::io` library. This function returns an instance of `std::io::Stdin`, which is a handle to the standard input of the program. This can then be used with functions such as `read_line` like in this program. I assume that usage techniques of these IO fucntions could vary so it is paramount to read the docs.

## Docs

Interestingly, you can generate an entirely custom documentation website from dependecies that you have in your program. We import the `rand` library crate into out project from [Crates.io](https://crates.io/) by modifying the `Cargo.toml` file. We can then run `cargo doc --open` to generate a local doc page with only the information we need for active dependencies. This is super cool! No digging through documentation related to things you arent using. Just everything you might need, all in one place. 
