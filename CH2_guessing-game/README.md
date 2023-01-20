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

## Result enums

The `Result` enum is Rust's way of handling error checking. Things return a `Result` enum if their failure needs to be handled. There is `Ok` which contains the value that you want to return and an `Err` value that gets returned when an error is thrown. This is not how other languages do things, so it is a mindset shift to think of enums as containers for things similar to structs. More will be seen in chapters to come.

## Print Placeholders

Using curly brackets `{}` within the `println` macro allows variables to be passed in. They can be filled with a variable in scope or empty to use args. Here is an example:

Input:
```
#![allow(unused)]
fn main() {
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
}
```
Output:
```
x = 5 and y = 12
```
## Random number generation

This was implemented to create random numbers for guessing. This was introduced in the context of importing crates in the `Cargo.toml` file.

## Comparison

Another operation that was imported from the standard library is comparison. The standard library comparison function returns an enum, allowing each comparison case to be handled. One thing that is crazy to me is the intelligence of the compiler. It knows that if two values are being compared and only one is expicitly typed, then the implicit one should recieve the same type. Take a look at the code in src to know what I am talking about.

