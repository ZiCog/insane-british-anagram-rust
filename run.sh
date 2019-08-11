#!/bin/bash

echo "iba-0:"
time ./target/release/iba-0 > anagrams-0.txt

echo "iba-2:"
time ./target/release/iba-2 > anagrams-2.txt

echo "iba-3:"
time ./target/release/iba-3 > anagrams-3.txt

echo "iba-4:"
time ./target/release/iba-4 > anagrams-4.txt

echo "iba-5:"
time ./target/release/iba-5 > anagrams-5.txt

echo "insane-british-anagram:"
time ./target/release/insane-british-anagram > anagrams.txt

