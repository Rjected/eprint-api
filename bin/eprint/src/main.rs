fn main() {
    if let Err(err) = eprint::cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
