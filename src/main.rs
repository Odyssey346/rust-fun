use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
    let mut integer = 0u32;
    let text:&str = "gamering";
    loop {
        integer += 1;
        while integer > 1 {
            let stdout = stdout();
            let message = String::from(text);
            let width = message.chars().count();
        
            let mut writer = BufWriter::new(stdout.lock());
            say(message.as_bytes(), width, &mut writer).unwrap();
        }
    }
}
