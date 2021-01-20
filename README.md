# murmur3-rs

![](https://img.shields.io/github/license/ABouttefeux/murmur3-rs)

Native rust implementation of [MurmurHash3](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp) that implement the trait [`std::hash::Hasher`](https://doc.rust-lang.org/std/hash/trait.Hasher.html)

MurmurHash3 is a non-crytographique Hasher. It is also not resistant to HashDoS. 
It is however faster than SipHash 1-3.
This implementation is intended for a light Hasher implementation to use with [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
with small keys.

## Usage


add the line `murmur_hash_3 = { version = "0.1.0", git = "https://github.com/ABouttefeux/murmur3-rs", branch = "main" }` in your Cargo.toml.
