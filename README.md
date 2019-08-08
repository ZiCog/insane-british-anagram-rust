# insane-british-anagram-rust
Rust program to find anagrams in the Debian british-english-insane dictionary file.

This is my first ever Rust program. You will find a number of different attempts at it here. By default it builds the latest fastest incarnation.

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
    
And visit http://localhost:8080/www/

## WTF ?!!

Do you notice the execution times above? How is it possible that it runs faster when built as WASM than as a native x86-64 executable.








