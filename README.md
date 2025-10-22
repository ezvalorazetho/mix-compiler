Mix Programming Language
========================

Mix is an experimental programming language that targets CPU-level performance while keeping the developer experience ergonomic. This repository hosts an early prototype written in Rust, featuring a lexer, parser, and AST builder.

Key Features
------------
- Lexer capable of recognizing Mix tokens (identifiers, literals, keywords, operators).
- Parser with panic-recovery so non-fatal errors do not immediately halt parsing.
- AST representation (`Node`) ready to feed later compilation stages such as type checking or code generation.
- Lightweight CLI (`mix`) for building Mix projects and printing the resulting AST.

Project Structure
-----------------
- `src/` – Compiler implementation (lexer, token, node, scanner) and CLI entry point.
- `test/src/` – Sample Mix programs used during parser development.
- `docs/` – Supporting documents and early language design notes.
- `Cargo.toml` & `Cargo.lock` – Rust project metadata.

Prerequisites
-------------
- Rust toolchain (latest stable recommended). Install via [rustup](https://rustup.rs/).

Build & Run
-----------
1. Run `cargo build` to compile the project and fetch dependencies.
2. Use `cargo run -- build <path>` to parse Mix sources. Example:
   ```
   cargo run -- build test
   ```
   The command reads `test/src/main.mx`, reports diagnostics (if any), and prints the generated AST.

Parser Testing
--------------
- Modify files under `test/src/` to exercise Mix syntax. The parser is designed to continue past non-fatal errors, making it easier to evaluate error recovery by intentionally introducing mistakes.

Contributing
------------
- Fork the repository and create a feature branch for your changes.
- Ensure `cargo fmt` and (optionally at this stage) `cargo clippy` pass before opening a pull request.
- Provide Mix code samples that demonstrate lexer/parser changes when relevant.

Short-term Roadmap
------------------
- Expand statement support (control flow, return, etc.).
- Introduce semantic analysis and type checking.
- Implement a backend for executable code generation.

License
-------
The license will be defined for the first public release. The project is currently an internal experimental effort.
