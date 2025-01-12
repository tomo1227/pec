fn main() {
    if let Err(code) = pec::run(std::env::args_os()) {
        std::process::exit(code);
    }
}
