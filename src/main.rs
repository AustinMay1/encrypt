mod algos;

use clap::Parser;
use crate::algos::rsa::{rsa_encrypt, rsa_decrypt};

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(clap::Parser)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,

    file: std::path::PathBuf,

    algo: Option<String>,

    key: Option<String>,
}

fn main() {

    let args = Args::parse();

    let mode = match &args.mode {
        Mode::Encrypt => "ENCRYPT",
        Mode::Decrypt => "DECRYPT",
    };

    //println!("{}", mode);
}