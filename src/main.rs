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

fn main() {
    let cli = Cli::parse();
    println!("name: {:?}", cli.hash);

    if let Some(algorithm) = cli.algorithm {
        println!("Value for algorithm: {:?}", algorithm);
    }
}
