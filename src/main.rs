
fn main() {
    if let Err(e) = catdog::get_args().and_then(catdog::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}