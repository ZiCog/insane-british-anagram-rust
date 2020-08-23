//
// insane-british-anagram.rs - Find words that have valid anagrams
//                             Words sourced from Debian's british-english-insane dictionary
//
// WARNING: This perhaps not the most readble way to write this algorithm, but it's fast!
//
// See: Bare Metal WASM by Cliff L Biffle:
//      https://users.rust-lang.org/t/writing-a-213-byte-webassembly-graphics-demo-with-rust/29099
//      http://cliffle.com/blog/bare-metal-wasm/

#![feature(test)]

#[cfg(unix)]
extern crate jemallocator;
#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern crate test;

use arrayvec::ArrayVec;
use hashbrown::HashMap; // Google's faster HashMap
use std::io::{self, Write};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
struct SliceSpec {
    begin: usize,
    end: usize,
}

#[derive(Clone)]
struct AnagramSet {
    word_slices: ArrayVec<[SliceSpec; 17]>,
}

impl AnagramSet {
    fn new(word: SliceSpec) -> AnagramSet {
        let mut word_slices = ArrayVec::new();
        word_slices.push(word);
        AnagramSet { word_slices }
    }
    fn push(&mut self, slice: SliceSpec) {
        self.word_slices.push(slice);
    }
}

#[cfg(not(feature = "web"))]
fn line_break() -> String {
    let br: String = String::from("\n");
    br
}

#[cfg(feature = "web")]
fn line_break() -> String {
    let br: String = String::from("<br>");
    br
}

fn output_anagrams(
    index: &[u64],
    anagram_map: &HashMap<u64, usize>,
    anagram_sets: &[AnagramSet],
    dictionary: &[u8],
) -> String {
    let mut output: String = std::string::String::from("");
    for hash in index {
        if let Some(anagram_sets_idx) = anagram_map.get(hash).copied() {
            let size = anagram_sets[anagram_sets_idx as usize].word_slices.len();
            if size > 1 {
                let mut separator = "";
                for (i, slice) in anagram_sets[anagram_sets_idx as usize]
                    .word_slices
                    .iter()
                    .enumerate()
                {
                    let begin = slice.begin;
                    let end = slice.end;
                    let word = &dictionary[begin..end];
                    output += separator;
                    output += &String::from_utf8_lossy(word);

                    if i == 0 {
                        separator = ": ";
                    } else {
                        separator = ", ";
                    }
                }
                output += &line_break();
            }
        }
    }
    output
}

fn find_anagrams(
    index: &mut Vec<u64>,
    anagram_map: &mut HashMap<u64, usize>,
    anagram_sets: &mut Vec<AnagramSet>,
    dictionary: &[u8],
) {
    let mut word_index = 0;
    let mut character_index = 0;
    let mut reject = false;
    let mut hash: u64 = 1;

    for &c in dictionary {
        if c.is_ascii_lowercase() {
            // We are scanning a valid word
            let prime_index = (c - b'a') as usize;
            hash = hash.wrapping_mul(PRIMES[prime_index].into());
            character_index += 1;
        } else if c == b'\n' {
            // We have hit the end of a word, use the word if it's valid
            if !reject {
                // Do we have a word with this key (potential anagram)?
                let word_spec = SliceSpec {
                    begin: word_index,
                    end: character_index,
                };
                match anagram_map.get_mut(&hash).copied() {
                    Some(idx) => {
                        // Found: Append it to the existing anagram set
                        anagram_sets[idx].push(word_spec);
                    }
                    None => {
                        // Not found: Add it to the map as start of new anagram set.
                        // Make a new anagram set with one word in it.
                        let anagram_set = AnagramSet::new(word_spec);
                        // Add the new anagram set to our list of anagram sets
                        anagram_map.insert(hash, anagram_sets.len());
                        anagram_sets.push(anagram_set);

                        // And add the new anagram set to index
                        index.push(hash);
                    }
                }
            }
            character_index += 1;
            word_index = character_index;
            hash = 1;
            reject = false;
        } else {
            // Invalid character
            hash = 1;
            reject = true;
            character_index += 1;
        }
    }
}

// One prime number for each lower case letter of the alphabet
static PRIMES: [u8; 26] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101,
];

pub fn anagrams(dictionary: &[u8]) -> String {
    // Container for sets of anagrams
    // An anagram set is simply an array of offets into the anagram_sets array
    let mut anagram_map = HashMap::with_capacity(376877);

    // Vector of AnagramSets
    let mut anagram_sets = Vec::with_capacity(376877);

    // An ordered index of anagram set keys
    let mut index = Vec::with_capacity(376877);

    find_anagrams(&mut index, &mut anagram_map, &mut anagram_sets, &dictionary);
    let output: String = output_anagrams(&index, &anagram_map, &anagram_sets, &dictionary);
    output
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn anagrams_html(s: String) -> String {
    let output: String = anagrams(s.as_bytes());
    output
}

// Called when the wasm module is instantiated
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

fn main() {
    match std::fs::read("/usr/share/dict/british-english-insane") {
        Ok(dictionary) => {
            let output = anagrams(&dictionary);
            let stdout = io::stdout();
            let mut stdout_handle = stdout.lock();
            match stdout_handle.write_all(output.as_bytes()) {
                Ok(()) => {}
                Err(e) => eprintln!("Error writing reult {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error reading dictionary: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_anagrams(b: &mut Bencher) {
        let dictionary = std::fs::read("/usr/share/dict/british-english-insane").unwrap();
        b.iter(|| anagrams(test::black_box(&dictionary)));
    }
}
