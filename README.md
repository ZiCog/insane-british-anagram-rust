# insane-british-anagram-rust
Rust program to find anagrams in the Debian british-english-insane dictionary file.

This is my first ever Rust program. You will find a number of different attempts at it here. By default it builds the latest fastest incarnation.



## Prerequisits

    $ sudo apt-get install curl
    $ sudo apt-get install libssl-dev
    $ curl https://sh.rustup.rs -sSf | sh
    $ source $HOME/.cargo/env
    $ cargo install wasm-bindgen-cli
    $ rustup target add wasm32-unknown-unknown

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

    $ python3 server.py
    
And visit http://localhost:9000/www/

## WTF ?!!

Do you notice the execution times above? How is it possible that it runs faster when built as WASM than as a native x86-64 executable.

## Tags

v0.1.1  Swapped HashMap to Google's HashBrown

    $ ./target/release/insane-british-anagram > anagrams.txt
    450ms
    429ms
    $ cd nodejs/
    $ node index.js > anagrams.txt
    517ms
    213ms









