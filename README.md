# fstream
A simple library to read/write files faster in Rust.

# Examples
#### Write text to a file.
```rust
match fstream::write_text("test_file.txt", "Hello world!", true) {

  Some(b) => println!("Number of bytes written to the file: {}", b),
  
  None => println!("Couldn't open or write to the file!"),
  
}
```

or just:
```rust
fstream::write_text("test_file.txt", "Hello world!", true).unwrap();
```

#### Read text from a file.
```rust
match fstream::read_text("test_file.txt") {

  Some(s) => println!("File content: {}", s),
        
  None => println!("Couldn't open or read the file"),
        
}
```

or just:
```rust
fstream::read_text("test_file.txt").unwrap();
```

# Functions
|Function|Description|
|--------|-----------|
|read|Reads file content into a buffer|
|read_text|Reads text from a file as a string|
|read_lines|Reads text from a file and stores lines into a vector of strings|
|read_words|Reads text from a file and stores words into a vector of strings|
|read_delim|Reads text from a file, splits it using a user specified delimiter and stores the tokens into a vector of strings|


# Installation

Add this line to your Cargo.toml:

```toml
[dependencies]
fstream = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate fstream
```
