# rsh - a minimal POSIX shell written in rust.

A minimal, interactive Unix shell built from scratch using Rust. This project was created as a learning exercise to understand the fundamental relationship between a shell, a terminal emulator, and the operating system kernel.

# What I Learned

## 1. Input Processing & Process Lifecycles

-   Created an infinite execution loop to keep the shell receptive to continuous commands.
-   Used standard input streams (`stdin`) to read user strings and trimmed trailing newline characters.
-   Used OS child process spawning and implemented blocking mechanisms so the shell waits for a command to finish before prompting again.

## 2. Argument Tokenization & Shell Built-ins

-   Split raw input strings on whitespace boundaries to separate the primary command from its trailing argument vectors.
-   Discovered why certain commands like `cd` and `exit` cannot be external binaries; they must modify the active environment or execution state of the parent shell process directly.
-   Implemented error handling to intercept bad user commands gracefully, preventing the parent shell process from crashing.

## 3. Multi-Command Pipelines

-   Explored how standard I/O redirection works using the pipe (`|`) character.
-   Learned how to chain sequential actions together by dynamically routing the standard output (`stdout`) of an upstream process straight into the standard input (`stdin`) of a downstream process.

# How to Run

Run the shell locally inside a native terminal environment to allow interactive standard inputs:

    cargo run

