mod algos;

use clap::Parser;

use crate::algos::rsa::{rsa_decrypt, rsa_encrypt};
#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(clap::ValueEnum, Clone)]
enum Algo {
    RSA,
}

#[derive(clap::Parser)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,

    file: std::path::PathBuf,

    #[arg(value_enum)]
    algo: Option<Algo>,

    key: Option<String>,
}

fn main() {
    let args = Args::parse();

    match &args.mode {
        Mode::Encrypt => rsa_encrypt(args.file),
        Mode::Decrypt => rsa_decrypt(args.file, args.key.unwrap()),
    };
}
