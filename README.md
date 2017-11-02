guessing_game
======
Game written in Rust where you try to guess the number that is randomly generated in as few tries as possible. First project in D7018E Special Studies in Embedded Systems, with focus on RUST.

## How to download, build and run.

1. `git clone https://github.com/superspindel/guessing_game.git` to clone repository.

2. `cd guessing_game` to navigate into guessing_game folder.

3. `cargo build --release` to build for release.

4. `cargo run`to run the game.

### Expected behavior.

1. When asked for input, an input that can not be parsed to a integer will result in an error message from parseIntError prepended with `in parsing u32, `.
