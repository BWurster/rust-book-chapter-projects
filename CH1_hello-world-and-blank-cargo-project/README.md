# Chapter 1: Getting Started

This chapter really just focuses on how to make a Rust program. It is broken into two primary parts:

1. Running a hello world program
2. Covering Cargo, the build system and package manager built into Rust.

## Hello, World!

The `hello-world` folder constains a single `main.rs` file with the following minimal contents.

```
fn main() {
    println!("Hello, world!");
}
```

This is the essence of a Rust program. Functions called main are special--they are the entry point of the program. Rust uses brackets to contain functions. Futhermore, our print statement is not a function; it is a macro. This will be covered later on in the book.

We can compile this program into an executable with the command `rustc main.rs` and then run it just like any other executable (`./main` on UNIX systems)

This way of compiling and running programs is very straightforward, but for larger projects, a project structure that handles version control and dependency management is nice to have. To fix this we have Cargo!

## Hello, Cargo!

To create a project with cargo we can run `cargo new {project name}`. This creates a new directory with the specified project name that contains a `Cargo.toml` file and a `src` directory to contain the working code. A hello world file is automatically added to this folder when it is generated, allowing for immediate use of Cargo.

One flag that could be useful is the `--vcs` flag. It defines what version control system is set up in the folder. Git is the default, but I could see `none` being useful too, especially if you have a git repository in a parent folder as is the case with this repo.

To build, use `cargo build`. This will put the executable in `./target/debug/`. If you want to build an optimized release version, use the `--release` flag and it will go in `./target/release/`.

Executing `cargo run` will run the version in the debug folder. Once again the `--release` flag will run the release executable.

`cargo check` is also useful for checking for successful compilation without producing an executable.
