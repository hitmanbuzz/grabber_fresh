mod source;
use std::io::{self, stdout, Write};

use crate::source::readm::readm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    

    println!("Grabber Made By Hitman|Moirangthem Henthoiba\n");
    print!("Enter Comic URL: ");
    stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();
    let _ = readm(&title).await;
    
    Ok(())
}
