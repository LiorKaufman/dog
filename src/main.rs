
fn main() {
    if let Err(e) = dog::get_args().and_then(dog::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}