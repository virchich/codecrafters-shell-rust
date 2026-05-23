[![progress-banner](https://backend.codecrafters.io/progress/shell/f3ef174c-adbf-4ef3-a1f4-998bca854a03)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# rust-shell

A POSIX-style shell implemented in Rust as a learning project through the CodeCrafters shell challenge.

## What this project demonstrates

- Lexing and parsing shell input into pipelines and command invocations
- Execution of built-in commands and external programs
- Output/error redirection and pipeline composition
- Background job tracking
- Simple shell state management for history, variable declaration, and completion records
- A REPL built on `rustyline`

## Implemented shell behavior

- Built-ins: `cd`, `pwd`, `echo`, `type`, `exit`, `history`, `jobs`, `declare`, `complete`
- Pipelines with `|`
- Background execution with `&`
- Redirection: `>`, `>>`, `2>`, `2>>`
- Basic argument parsing with quotes and escapes
- Variable expansion for declared values

## Technical notes

- Language: Rust
- Edition: 2021
- Primary dependencies: `rustyline`, `regex`
- Structure:
    - `src/syntax`: lexer, tokens, parser
    - `src/commands`: built-ins, expansion, execution, redirection, command resolution
    - `src/state`: in-memory stores for history, jobs, declarations, completions
    - `src/repl`: interactive shell session

## Test coverage

The unit test suite focuses on the parts most likely to cause behavioral regressions:

- lexical scanning
- parsing
- argument expansion
- redirection and executable resolution
- state stores
- selected built-ins with meaningful logic

Current project-source coverage is above 60%.

## Running locally

```sh
./your_program.sh
```

Run tests with:

```sh
cargo test
```

## Why this repo exists

This is not intended to be a production shell. It is a systems programming exercise focused on parser design, process
orchestration, and building a clean command execution flow in Rust.
