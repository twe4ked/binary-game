use binary_game;

fn main() {
    binary_game::run().unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        std::process::exit(1);
    });
}
