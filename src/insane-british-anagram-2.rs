//
// insane-british-anagram-2.rs - Find words that have valid anagrams
//                               Words sourced from Debian's british-english-insane dictionary
//
// heater - 2019-07-30
// 

#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;
use std::io::{BufReader,BufRead};

fn validWord (word : &String) -> bool {
    let bytes = word.as_bytes();
    for c in bytes {
        if (*c < 'a' as u8) || (*c > 'z' as u8) {
            return false;
        }
    }
    return true;
}

fn primeHash (word: &String) -> u64 {
    // One prime number for each lower case letter of the alphabet
    let primes: [u64; 26] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];

    let slice = word.as_bytes();
    let mut hash: u64 = 1;
    for c in slice {
        let index = (c - 97) as usize;
        hash = hash.wrapping_mul(primes[index])
    }
    return hash;
}

fn main() {
    let stdout = io::stdout();
    let mut stdoutHandle = stdout.lock();

    // All the words read from the dictionary
    let mut words: Vec<String> = Vec::new();

    // Map container for sets of anagrams 
    let mut anagramMap: HashMap<u64, Vec<usize>> = HashMap::new();

    // An ordered index of anagram set keys 
    let mut index: Vec<u64> = Vec::new();

    // Read all words from dictionary into a word list
    let file = File::open("/usr/share/dict/british-english-insane").unwrap();
    for line in BufReader::new(file).lines() {
        let word = line.unwrap();
        words.push(word);
    }

    let mut wordNo = 0;
    for word in &words {
        if validWord(&word) {
            let hash = primeHash(&word);

            // Do we have a word with this key (potential anagram)?
            match anagramMap.get_mut(&hash) {
                Some(anagramSet) => {
                    // Found: Append it to the existing anagram set
                    anagramSet.push(wordNo);
                },
                None => {
                    // Not found: Add it to the map as start of new anagram set.
                    let mut anagramSet: Vec<usize> = Vec::new();
                    anagramSet.push(wordNo);
                    anagramMap.insert(hash, anagramSet);

                    // And add the new anagram set to index
                    index.push(hash);
                }
            }
        }
        wordNo = wordNo + 1;
    }

    let mut output: String = "".to_string();
    for hash in index {
        match anagramMap.get(&hash) {
            Some(anagramSet) => {
                if anagramSet.len() > 1 {
                    output = output + &words[anagramSet[0 as usize]];
                    let mut separator = ": ";
                    for wordNo in &anagramSet[1..] {
                        output = output + &separator;
                        output = output + &words[*wordNo];
                        separator = ", ";
                    }
                    output = output + "\n";
                }
            },
            _ => (),
        }
    }
    
    match stdoutHandle.write_all(output.as_bytes()) {
        Ok(()) => (),
        Err(e) => println!("Error writing reult {}", e),
    }
}

