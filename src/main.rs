use img_hash_linker::compute_hash_from_file;
use img_hash_linker::open_link_from_image_file;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <image_path> [csv_dict_path]", args[0]);
        eprintln!("  - With only image_path: computes and displays the hash");
        eprintln!("  - With both arguments: opens the link associated with the image");
        process::exit(1);
    }

    let image_path: String = args.get(1).unwrap().clone();

    if args.len() >= 3 {
        let dict_path: String = args.get(2).unwrap().clone();
        open_link_from_image_file(image_path, dict_path);
    } else {
        match compute_hash_from_file(image_path) {
            Ok(hash) => println!("Image hash: {}", hash),
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}
