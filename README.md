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
