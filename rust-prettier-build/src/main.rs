//! Entry point: parse arguments, run the game, report the exit code.

fn main() {
    let code = tetris::app::main_with(std::env::args().skip(1));
    std::process::exit(code);
}
