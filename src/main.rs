use anyhow::Result;

fn main() {
    if let Err(e) = wasminator::run() {
        // eprintln!("{}", e);
        std::process::exit(1);
    }
}
