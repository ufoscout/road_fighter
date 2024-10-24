export RUST_BACKTRACE := "full"

# Lists all the available commands
default:
  @just --list

# Run the game in development mode with the bevy dynamic linking feature enabled
run:
  @cargo run --features bevy/dynamic_linking

# Run the game in development mode with the bevy dynamic linking feature enabled
test test_name="":
  @cargo test {{test_name}} --features bevy/dynamic_linking

# Build the game in release mode
build:
  @cargo build --release