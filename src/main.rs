use img_hash_linker::compute_ahash_from_file;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        process::exit(1);
    }

    let image_path = &args[1];

    match compute_ahash_from_file(image_path) {
        Ok(hash) => println!("Image hash: {}", hash),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
