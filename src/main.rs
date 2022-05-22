use std::{io, io::prelude::*};

use clap::{ArgEnum, Parser};

/// Pipe stdin to stdout if stdin matches a hash
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Expected hash of stdin file
    #[clap(forbid_empty_values = true)]
    hash: String,

    /// The hashing algorithm used for the hash
    #[clap(arg_enum, short, long)]
    algorithm: Option<HashAlgorithm>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum HashAlgorithm {
    Md5,
    Sha256,
    Sha512,
}

fn get_hash_function(hash: String) -> Result<HashAlgorithm, &'static str> {
    match hash.len() {
        32 => Ok(HashAlgorithm::Md5),
        64 => Ok(HashAlgorithm::Sha256),
        128 => Ok(HashAlgorithm::Sha512),
        _ => Err("Could not detect hash algorithm"),
    }
}

fn main() {
    let cli = Cli::parse();
    println!("Hash: {:?}", cli.hash);

    if let Some(algorithm) = cli.algorithm {
        println!("Value for algorithm: {:?}", algorithm);
    } else {
        let algorithm = get_hash_function(cli.hash).unwrap();
        println!("Detected value for algorithm: {:?}", algorithm);
    }

    let lines: String = io::stdin().lock().lines().fold(String::new(), |s, l| s + &l.unwrap() + "\n");

    let digest = md5::compute(lines);
    let digest_str = format!("{:x}", digest);
    println!("{}", digest_str);
}
