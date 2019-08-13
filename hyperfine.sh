#!/bin/bash

hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches || true'  'target/release/iba-0 > anagrams-0.txt'

hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches || true'  'target/release/iba-2 > anagrams-2.txt'

hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches || true'  'target/release/iba-3 > anagrams-3.txt'

hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches || true'  'target/release/iba-4 > anagrams-4.txt'

hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches || true'  'target/release/iba-5 > anagrams-5.txt'

hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches || true'  'target/release/insane-british-anagram > anagrams.txt'


