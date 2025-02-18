use std::iter;
use std::io::{self,Write};
use std::thread;
use std::time::Duration;
use termion;

fn main() {
    let size = termion::terminal_size().unwrap().0 as usize;
    let mut text : String = iter::repeat(' ').take(size).collect();
    print!("{text}");
    io::stdout().flush().unwrap();
    loop {
        for c in b"# " {
            for i in 0..size {
                unsafe{
                    text.as_bytes_mut()[i] = *c;
                }
                print!("\r{text}");
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(5));
            }
        }
    }   
}
