# insane-british-anagram-rust
Rust program to find anagrams in the Debian british-english-insane dictionary file.

This is my first ever Rust program. You will find a number of different attempts at it here. By default it builds the latest fastest incarnation.



## Prerequisits

    $ sudo apt-get install curl
    $ sudo apt-get install libssl-dev
    $ sudo apt-get install make
    $ curl https://sh.rustup.rs -sSf | sh
    $ source $HOME/.cargo/env
    $ cargo install wasm-bindgen-cli --version 0.2.67
    $ rustup target add wasm32-unknown-unknown

Note: The version of wasm-bindgen-cli must match the version of wasm-bingen specified in Cargo.toml. Be sure to change both if upgrading.


## Build

The build.sh script will build the anagram finder as a native executable and as WASM for node.js and the web.

    $ cargo clean
    $ ./build.sh
    
## Run

The native executable: (Runs the anagram finder twice and prints execution time to stderr.

    $ ./target/release/insane-british-anagram > anagrams.txt
    496ms
    477ms

Under node.js

    $ cd nodejs
    $ node index.js > anagrams.txt
    592ms
    289ms

As a web page:

    $ ./target/release/insane_http
    
And visit http://localhost:9000/www/

## WTF ?!!

Do you notice the execution times above? How is it possible that it runs faster when built as WASM than as a native x86-64 executable.

## Tags

v0.1.1  Swapped HashMap to Google's HashBrown:

    $ ./target/release/insane-british-anagram > anagrams.txt
    450ms
    429ms
    $ cd nodejs/
    $ node index.js > anagrams.txt
    517ms
    213ms

v0.1.2 Optimizations by kornelski:

    $ ./target/release/insane-british-anagram > anagrams.txt
    328ms
    326ms
    $ node index.js > anagrams.txt
    490ms
    324ms

v0.1.3 Using jemallocator, with spectacular results!

    $ time ./target/release/insane-british-anagram > anagrams.txt
    324ms
    79ms


## Timings on a Raspberry Pi 3 and Buster:

v0.1.0 Using std HashMap:

    $ ./target/release/insane-british-anagram > anagrams.txt
    729ms
    717ms

v0.1.1 Using Goggle's HashBrown:

    $ ./target/release/insane-british-anagram > anagrams.txt
    606ms
    583ms

v0.1.2 Using ArrayVec instead of std::Vector and optimizations by kornelski:

    $ ./target/release/insane-british-anagram > anagrams.txt
    664ms
    639ms

v0.1.3 Using jemallocator:

    $ time target/release/insane-british-anagram > anagrams.txt
    664ms
    571ms








