
//! Implementation of [MurmurHash3](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp)
//! that implement the trait [`std::hash::Hasher`]
//!
//! MurmurHash3 is a non-crytographique Hasher. It is also not resistant to HashDoS. 
//! It is however faster than SipHash 1-3.
//! This implementation is intended for a light Hasher implementation to use with [`std::collections::HashMap`]
//! with small keys.

extern crate rand;

use std::{
    hash::{Hasher, BuildHasher},
    collections::{HashMap, HashSet},
    num::Wrapping,
    convert::TryInto,
    iter::Iterator,
};
use rand::Rng;

/// HashMap using Murmur3.
pub type Murmur3HashMap<K, V> = HashMap<K, V, BuildMurmur>;
/// HashSet using Murmur3.
pub type Murmur3HashSet<T> = HashSet<T, BuildMurmur>;

/// Builder for [`Murmur3`]. The builder generate a random seed, and all Hasher 
/// generated from a instance of the builder will share the same seed.
pub struct BuildMurmur {
    seed: u32,
}

impl BuildMurmur {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {seed: rng.gen()}
    }
}

impl BuildHasher for BuildMurmur {
    type Hasher = Murmur3;
    fn build_hasher(&self) -> Self::Hasher{
        Murmur3::new(self.seed)
    }
}

/// Murmur3 represent the state of the murmur3 hash algorithm. It does NOT represent a hash, but a
/// partial computation of some data. To get the final hash from everything we wrote into the
/// Hasher, we need to call [`Murmur3::finish()`].
///
/// Every time we use [`Murmur3::write()`] on an instance of [`Murmur3`] it agregates data. See it as a lazily
/// evaluated hash.
pub struct Murmur3 {
    // remaining bytes from last call to write()
    rem: [u8; 4],
    rem_len: usize,

    // the total length of everything we wrote
    total_len: usize,

    // the hash state
    hash: Wrapping<u32>,
}

impl Murmur3 {
    pub fn new(seed: u32) -> Self {
        Self{
            rem: [0, 0, 0, 0],
            rem_len: 0,
            total_len: 0,
            hash: Wrapping(seed),
        }
    }
}

impl Murmur3 {
    fn compute_full_word(&mut self, k:u32) {
        let k = Wrapping(k);
        self.hash ^= murmur_32_scramble(k);
        self.hash = (self.hash << 13) | (self.hash >> 19);
        self.hash = self.hash * Wrapping(5) + Wrapping(0xe6546b64);
    }
}

impl Hasher for Murmur3 {
    fn finish(&self) -> u64 {
        let mut h = self.hash;
        let len = Wrapping((self.total_len + self.rem_len) as u32);

        // remaining bytes are always treated as little-endian
        // (full words are treated as native endian bytes)
        let k = Wrapping(u32::from_le_bytes(self.rem));

        h ^= murmur_32_scramble(k);
        h ^= len;
        h ^= h >> 16;
        h = h * Wrapping(0x85ebca6b_u32);
        h ^= h >> 13;
        h = h * Wrapping(0xc2b2ae35_u32);
        h ^= h >> 16;

        // h.0 gives me the unwrapped element
        return h.0 as u64;
    }

    fn write(&mut self, bytes: &[u8]) {
        // Since we can compute bytes 4 by 4, we perform the "full word" computation here and we
        // remember the remaining 1-3 bytes if any. Each time we re-call write, we do not start
        // from the "bytes" slice, but from the remainder.

        // Fills remainder array if not empty
        let to_get = 4 - self.rem_len;
        if self.rem_len > 0 {
            for n in &bytes[0..to_get] {
                self.rem[self.rem_len] = *n;
                self.rem_len += 1;
            }

            self.compute_full_word(u32::from_ne_bytes(self.rem));
            self.rem_len = 0;
            self.total_len += 4;
        }

        // compute the remaining 4byte-wide words
        let mut iter = bytes[to_get..].chunks_exact(4);
        while let Some(chunk) = iter.next() {
            // normally safe because chunk's len is always 4
            let k = u32::from_ne_bytes(chunk.try_into().unwrap());
            self.compute_full_word(k);
        }

        // store remaining bytes 
        let rem = iter.remainder();
        self.total_len += bytes.len() - rem.len();
        for n in rem {
            self.rem[self.rem_len] = *n;
            self.rem_len += 1;
        }
    }
}

fn murmur_32_scramble(k: Wrapping<u32>) -> Wrapping<u32> {
    let k = k * Wrapping(0xcc9e2d51_u32);
    let k = (k << 15) | (k >> 17);
    let k = k * Wrapping(0x1b873593_u32);
    return k;
}
