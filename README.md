# fstream
A simple library to read/write files faster in Rust.

# Examples
Write text to a file.
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

Read text from a file.
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

# Installation

Add this line to your Cargo.toml:

```rust
[dependencies]
fstream = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate fstream
```
