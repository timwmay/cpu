let ref opcodes: HashMap<u8, opcodes::OpCode> = *opcodes::OPCODES_MAP;

# VSCode docs
https://code.visualstudio.com/docs/languages/rust

# Rust commands
    rustc --version
    rustup update

# Rust Compiler (Cargo)
    cargo new hello_world   # Create a new project
    cargo build             # Build the project
    cargo run               # Run the project

# SDL Library
    after adding SDL2 to the Cargo.toml file, run:
    export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"