use std::env;
use std::process;

use img_hash_linker::algorithm::hash_proximity::try_finding_similar_hash;
use img_hash_linker::compute_hash;
use img_hash_linker::data_handle::load_csv::load_data_from_csv;
use img_hash_linker::open_link_from_hash;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <image_path> [csv_dict_path]", args[0]);
        eprintln!("  - With only image_path: computes and displays the hash");
        eprintln!("  - With both arguments: opens the link associated with the image");
        process::exit(1);
    }

    let image_path: String = args.get(1).unwrap().clone();
    let hash: String = compute_hash(image::open(image_path).unwrap(), true, None).unwrap();

    if args.len() >= 3 {
        let dict_path: String = args.get(2).unwrap().clone();
        let links: Vec<(String, String)> = load_data_from_csv(dict_path).unwrap();

        match open_link_from_hash(links.clone(), hash.clone()) {
            Ok(message) => println!("{}", message),
            Err(e) => match try_finding_similar_hash(hash.clone(), links.clone(), None) {
                Ok((similar_hash, _link, proximity)) => {
                    println!(
                        "{} (Proximity: {:.2}%)",
                        open_link_from_hash(links.clone(), similar_hash).unwrap(),
                        proximity * 100.0
                    );
                }
                Err(_) => {
                    println!("{}", e);
                }
            },
        }
    } else {
        println!("Image hash: {}", hash);
    }
}
