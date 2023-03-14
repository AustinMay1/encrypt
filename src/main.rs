use clap::Parser;

enum Mode {
    Encrypt(String),
    Decrypt(String),
}

#[derive(Parser, Debug)]
struct Args {
    mode: String,
    file: std::path::PathBuf,
    algo: Option<String>,
    key: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);    
}