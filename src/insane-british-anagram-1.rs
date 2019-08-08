//
// insane-british-anagram-2.rs - Find words that have valid anagrams
//                               Words sourced from Debian's british-english-insane dictionary
//
// heater - 2019-07-29
// 

#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;
use std::io::{BufReader,BufRead};

// Calculates n!
fn factorial(n: usize) -> Int {
   let mut a = Int::from(1);

   for i in 2..n {
       a *= i;
   }

   return a * n;
}

fn validWord (word : &String) -> bool {
    let bytes = word.as_bytes();
    for c in bytes {
        if (*c < 'a' as u8) || (*c > 'z' as u8) {
            return false;
        }
    }
    return true;
}

fn primeHash (word: &String) -> Int {
    // One prime number for each lower case letter of the alphabet
    let primes: [u64; 26] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];

    let slice = word.as_bytes();
    let mut hash = Int::from(1);
    for c in slice {
        let index = (c - 97) as usize;  
    	hash = hash * primes[index];
    }
    return hash;
}

fn main() {
    let stdout = io::stdout();
    let mut stdoutHandle = stdout.lock();

    // Map container for sets of anagrams 
    // An anagram set is simply a vector of pointers to word strings
    let mut anagramMap: HashMap<Int, Vec<String>> = HashMap::new();

    // An ordered index of anagram set keys 
    let mut index: Vec<Int> = Vec::new();

    let file = File::open("/usr/share/dict/british-english-insane").unwrap();
    for line in BufReader::new(file).lines() {
        let word = line.unwrap();

        if validWord(&word) {
            let hash = primeHash(&word);
            //let hash2 = primeHash(&word);

            // Do we have a word with this key (potential anagram)?
            match anagramMap.get_mut(&hash) {
                Some(anagramSet) => {
                    // Found: Append it to the existing anagram set
                    anagramSet.push(word);
                },
                None => {
                    // Not found: Add it to the map as start of new anagram set.
                    let mut anagramSet: Vec<String> = Vec::new();
                    anagramSet.push(word);
                    anagramMap.insert(hash, anagramSet);

                    // And add the new anagram set to index
                    index.push(hash);
                }
            }
        }
    }

    let mut output: String = "".to_string();
    for hash in index {
        match anagramMap.get(&hash) {
            Some(anagramSet) => {
                if anagramSet.len() > 1 {
                    output = output + &anagramSet[0];
                    let mut separator = ": ";
                    for word in &anagramSet[1..] {
                        output = output + &separator;
                        output = output + &word;
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

