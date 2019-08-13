//
// insane-british-anagram-4.rs - Find words that have valid anagrams
//                               Words sourced from Debian's british-english-insane dictionary
//
// heater - 2019-08-01
//
// WARNING: This is not a good solution. Only a crazy experiment in trying to write Rust like C.
//          It's verbose, complex and slow!

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

#[derive(Copy, Clone)]
struct SliceSpec {
    begin: usize,
    end: usize,
}

#[derive(Copy, Clone)]
struct AnagramSet {
    wordSlices: [SliceSpec; 17],
    size: usize,
}

impl AnagramSet {
    fn new(word: SliceSpec) -> AnagramSet {
        return AnagramSet {
            wordSlices: [word; 17],
            size: 1,
        };
    }
    fn push(&mut self, slice: SliceSpec) {
        self.wordSlices[self.size] = slice;
        self.size = self.size + 1;
    }
}

fn readInsaneBritishDictionary(mut dictionary: &mut Vec<u8>) -> std::io::Result<()> {
    let mut file = File::open("/usr/share/dict/british-english-insane")?;
    file.read_to_end(&mut dictionary)?;
    return Ok(());
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
    return hash;
}

fn isLowerCase(c: &u8) -> bool {
    if (*c < 'a' as u8) || (*c > 'z' as u8) {
        return false;
    } else {
        return true;
    }
}

fn anagrams() {
    let stdout = io::stdout();
    let mut stdoutHandle = stdout.lock();

    // Container for sets of anagrams
    // An anagram set is simply an array of offets into the anagramSets array
    let mut anagramMap: HashMap<u64, usize> = HashMap::new();

    // Vector of AnagramSets
    let mut anagramSets: Vec<AnagramSet> = Vec::new();
    let mut anagramSetsCount: usize = 0;

    // An ordered index of anagram set keys
    let mut index: Vec<u64> = Vec::new();

    let mut dictionary = Vec::new();

    match readInsaneBritishDictionary(&mut dictionary) {
        // Takes 25ms on PC
        Ok(()) => {
            let mut wordIndex = 0;
            let mut characterIndex = 0;
            let mut reject = false;
            for c in &dictionary {
                if isLowerCase(&c) {
                    // We are scanning a valid word
                    characterIndex = characterIndex + 1;
                } else if *c == '\n' as u8 {
                    // We have hit the end of a word, use the word if it's valid
                    if !reject {
                        // Do we have a word with this key (potential anagram)?
                        let word = &dictionary[wordIndex..characterIndex];

                        let hash = primeHash(&word);

                        let wordSpec = SliceSpec {
                            begin: wordIndex,
                            end: characterIndex,
                        };
                        match anagramMap.get_mut(&hash) {
                            Some(anagramSetsCount) => {
                                // Found: Append it to the existing anagram set
                                anagramSets[*anagramSetsCount].push(wordSpec);
                            }
                            None => {
                                // Not found: Add it to the map as start of new anagram set.
                                // Make a new anagram set with one word in it.
                                let anagramSet = AnagramSet::new(wordSpec);
                                // Add the new anagram set to our list of anagram sets
                                anagramSets.push(anagramSet);
                                anagramMap.insert(hash, anagramSetsCount);
                                anagramSetsCount = anagramSetsCount + 1;

                                // And add the new anagram set to index
                                index.push(hash);
                            }
                        }
                    }
                    characterIndex = characterIndex + 1;
                    wordIndex = characterIndex;
                    reject = false;
                } else {
                    // Invalid character
                    reject = true;
                    characterIndex = characterIndex + 1;
                }
            }

            let mut output: String = "".to_string();
            for hash in index {
                match anagramMap.get(&hash) {
                    Some(AnagramSetsCount) => {
                        let size = anagramSets[*AnagramSetsCount as usize].size;
                        if size > 1 {
                            let mut separator = "";
                            let mut i = 0;
                            while i < size {
                                let begin = anagramSets[*AnagramSetsCount].wordSlices[i].begin;
                                let end = anagramSets[*AnagramSetsCount].wordSlices[i].end;
                                let slice = &dictionary[begin..end];
                                let word = String::from_utf8_lossy(&slice).to_string();
                                output = output + &separator;
                                output = output + &word;

                                if i == 0 {
                                    separator = ": ";
                                } else {
                                    separator = ", ";
                                }
                                i = i + 1;
                            }
                            output = output + "\n";
                        }
                    }
                    _ => (),
                }
            }

            match stdoutHandle.write_all(output.as_bytes()) {
                Ok(()) => {}
                Err(e) => println!("Error writing reult {}", e),
            }
        }
        Err(e) => {
            println!("Error reading dictionary: {}", e);
        }
    }
}

fn main() {
    anagrams();
}
