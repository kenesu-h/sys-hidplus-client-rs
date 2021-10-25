# Build
This documentation is intended to help you through the steps of building the
client from source. You may be doing this because you want to work directly
with the source code, or because no release exists for your current operating
system.

This documentation will tell you how to build the client from source, whether
because no release exists for your operating system or you want to work with the
code.

# Requirements:
You need the following installed on your computer:
- A terminal/shell such as Command Prompt, PowerShell, Alacritty, etc.
- Rust installed, preferably using [rustup](https://rustup.rs/).
- [Git](https://git-scm.com/) installed.
- The SDL2 development libraries.
  - [rust-sdl2's README](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md#requirements)
    is best at telling you how to install it.

# Directions
1. Clone this repo using Git.
  - If you have made your own fork, clone your fork instead.
2. Run `cargo build`. You should have an executable for your operating system in
   `target/debug`.
   - Alternatively, you can run (and build) by running `cargo run`.

You'll probably want to switch to a branch other than `main`, since `main` is 
one intended for legacy and stability. As a result, it's missing a lot of 
features.

If you don't know how to use Git and would like to switch branches, read the 
following:

You can use `git checkout 'branch_name'`, where 'branch_name' is the name of the
branch you want to switch to. For example, if you wanted to switch to
`1.1.0-alpha`, run `git checkout 1.1.0-alpha`. If you wanted to switch back to
`main`, run `git checkout main`.