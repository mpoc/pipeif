use std::{io, io::prelude::*};

use clap::{ArgEnum, Parser};
use sha2::{Sha256, Sha512, Digest};

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

    let md5_digest = md5::compute(&lines);
    let md5_digest_str = format!("{:x}", md5_digest);
    println!("{}", md5_digest_str);

    let mut sha256_digest = Sha256::new();
    sha256_digest.update(&lines);
    let sha256_digest_str: String = format!("{:X}", sha256_digest.finalize());
    println!("{}", sha256_digest_str);

    let mut sha512_digest = Sha512::new();
    sha512_digest.update(&lines);
    let sha512_digest_str: String = format!("{:X}", sha512_digest.finalize());
    println!("{}", sha512_digest_str);
}
