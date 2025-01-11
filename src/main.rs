use sha2::{Sha256, Digest};
use clap::Parser;
use rayon::prelude::*;

/// # FIELDS
/// - n -> numbers of zeros at the end of the hash.
/// - f -> number of results to find.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    n: usize,
    #[clap(short, long)]
    f: usize,
}

fn main() {
    let args = Args::parse();
    let zeros = "0".repeat(args.n);

    let results = (1..)
        .into_iter()
        .par_bridge() // I AM SPEED
        .filter_map(move |num| {
            let mut hasher = Sha256::new();
            hasher.update(num.to_string());
            let result = hasher.finalize();
            let hash = format!("{:x}", result);
            if hash.ends_with(&zeros) {
                Some((num, hash))
            } else {
                None
            }
        })
        .take_any(args.f) // rayons take
        .collect::<Vec<_>>();

    for (num, hash) in results {
        println!("{}, \"{}\"", num, hash);
    }
}
