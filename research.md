### Tested using the `image` crate for image resizing.

Using `hyperfine --warmup 10 --runs 500 "./target/release/img_hash_linker <image>"`.
The mean time to get the hash were 57.9 ms ± 2.5 ms

### Tested using the `fast_image_resize` crate for image resizing.

Using `hyperfine --warmup 10 --runs 500 "./target/release/img_hash_linker <image>"`.
The mean time to get the hash were 45.3 ms ± 0.8 ms

### Checking the difference:

The `fast_image_resize` crate improved the overall hashing speed in 12.6ms
