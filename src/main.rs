use std::iter;
use std::io::{self,Write};
use std::thread;
use std::time::Duration;

fn main() {
    print!("size: ");
    io::stdout().flush().unwrap();
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Expected a string");
    let size = size[..size.len()-1].parse::<usize>().expect("Expected a non-negative integer.");
    let mut text : String = iter::repeat(' ').take(size).collect();
    print!("{text}");
    io::stdout().flush().unwrap();
    loop {
        for c in b"# " {
            for i in 0..size {
                unsafe{
                    text.as_bytes_mut()[i] = *c;
                    print!("\r{text}");
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_millis(5));
                }
            }
        }
    }   
}
