
extern crate rand;

use std::{
    hash::{Hasher, BuildHasher},
    collections::{HashMap, HashSet},
    vec::Vec,
    num::Wrapping,
    convert::TryInto,
    iter::Iterator,
};
use rand::Rng;

pub type Murmur3HashMap<K, V> = HashMap<K, V, BuildMurmur>;
pub type Murmur3HashSet<T> = HashSet<T, BuildMurmur>;

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

pub struct Murmur3 {
    bytes: Vec<u8>,
    seed: u32,
}

impl Murmur3 {
    pub fn new(seed: u32) -> Self {
        Self{
            bytes: Vec::new(),
            seed,
        }
    }
}

impl Hasher for Murmur3 {
    fn finish(&self) -> u64{
        let mut h = Wrapping(self.seed);
        let len = self.bytes.len();
        /* Read in groups of 4. */
        for i in (0..len).step_by(4).rev() {
            let k = Wrapping(u32::from_be_bytes(self.bytes[i..i + 4].try_into().unwrap()));
            h ^= murmur_32_scramble(k);
            h = (h << 13) | (h >> 19);
            h = h * Wrapping(5_u32) + Wrapping(0xe6546b64_u32);
        }
        let mut k: Wrapping<u32> = Wrapping(0);
        let padding = len % 4;
        for i in (1..padding + 1).rev() {
            k <<= 8;
            k |= Wrapping(self.bytes[(len - padding)  + i] as u32);
        }
        h ^= murmur_32_scramble(k);
    	h ^= Wrapping(len as u32);
    	h ^= h >> 16;
    	h = h * Wrapping(0x85ebca6b_u32);
    	h ^= h >> 13;
    	h = h * Wrapping(0xc2b2ae35_u32);
    	h ^= h >> 16;
        // h.0 gives me the unwrapped element
    	return h.0 as u64;
    }
    
    fn write(&mut self, bytes: &[u8]){
        self.bytes.extend_from_slice(bytes)
    }
}

fn murmur_32_scramble(k: Wrapping<u32>) -> Wrapping<u32> {
    let k = k * Wrapping(0xcc9e2d51_u32);
    let k = (k << 15) | (k >> 17);
    let k = k * Wrapping(0x1b873593_u32);
    return k;
}   
