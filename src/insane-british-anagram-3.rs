//
// insane-british-anagram-3.rs - Find words that have valid anagrams
//                               Words sourced from Debian's british-english-insane dictionary
//
// heater - 2019-07-30
//
#![allow(non_snake_case)]

#[cfg(unix)]
extern crate jemallocator;
#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

struct SliceSpec {
    begin: usize,
    end: usize,
}

fn readInsaneBritishDictionary(mut dictionary: &mut Vec<u8>) -> std::io::Result<()> {
    let mut file = File::open("/usr/share/dict/british-english-insane")?;
    file.read_to_end(&mut dictionary)?;
    Ok(())
}

fn primeHash(slice: &[u8]) -> u64 {
    // One prime number for each lower case letter of the alphabet
    let primes: [u64; 26] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101,
    ];

    let mut hash: u64 = 1;
    for c in slice {
        let index = (c - 97) as usize;
        hash = hash.wrapping_mul(primes[index]);
    }
    hash
}

fn isLowerCase(c: &u8) -> bool {
    !((*c < b'a') || (*c > b'z'))
}

fn insaneBritishAnagram() {
    let stdout = io::stdout();
    let mut stdoutHandle = stdout.lock();

    // Map container for sets of anagrams
    // An anagram set is simply a vector of pointers to word strings
    let mut anagramMap: HashMap<u64, Vec<SliceSpec>> = HashMap::new();

    // An ordered index of anagram set keys
    let mut index: Vec<u64> = Vec::new();

    let mut dictionary = Vec::new();
    match readInsaneBritishDictionary(&mut dictionary) {
        Ok(()) => {
            let mut wordIndex = 0;
            let mut characterIndex = 0;
            let mut reject = false;
            for c in &dictionary {
                if isLowerCase(&c) {
                    // We are scanning a valid word
                    characterIndex += 1;
                } else if *c == b'\n' {
                    // We have hit the end of a word, use the word if it's valid
                    if !reject {
                        // Do we have a word with this key (potential anagram)?
                        let word = &dictionary[wordIndex..characterIndex];
                        let hash = primeHash(&word);
                        //let string = String::from_utf8_lossy(&word).to_string();
                        let wordSpec = SliceSpec {
                            begin: wordIndex,
                            end: characterIndex,
                        };
                        match anagramMap.get_mut(&hash) {
                            Some(anagramSet) => {
                                // Found: Append it to the existing anagram set
                                anagramSet.push(wordSpec);
                            }
                            None => {
                                // Not found: Add it to the map as start of new anagram set.
                                let mut anagramSet: Vec<SliceSpec> = Vec::new();
                                anagramSet.push(wordSpec);
                                anagramMap.insert(hash, anagramSet);

                                // And add the new anagram set to index
                                index.push(hash);
                            }
                        }
                    }
                    characterIndex += 1;
                    wordIndex = characterIndex;
                    reject = false;
                } else {
                    // Invalid character
                    reject = true;
                    characterIndex += 1;
                }
            }

            let mut output: String = "".to_string();
            for hash in index {
                if let Some(anagramSet) = anagramMap.get(&hash) {
                    if anagramSet.len() > 1 {
                        let mut slice = &dictionary[anagramSet[0].begin..anagramSet[0].end];
                        let mut word = String::from_utf8_lossy(&slice);
                        output = output + &word;
                        let mut separator = ": ";
                        for wordSlice in &anagramSet[1..] {
                            slice = &dictionary[wordSlice.begin..wordSlice.end];
                            word = String::from_utf8_lossy(&slice);
                            output += separator;
                            output = output + &word;
                            separator = ", ";
                        }
                        output += "\n";
                    }
                }
            }

            match stdoutHandle.write_all(output.as_bytes()) {
                Ok(()) => (),
                Err(e) => println!("Error writing reult {}", e),
            }
        }
        Err(e) => {
            println!("Error reading dictionary: {}", e);
        }
    }
}

fn main() {
    insaneBritishAnagram();
}
