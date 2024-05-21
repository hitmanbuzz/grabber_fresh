mod source;
use std::io::{self, stdout, Write};

use crate::source::readm::readm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Grabber Made By Moirangthem Henthoiba\n");
    print!("Enter Comic URL: ");
    stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();
    let _ = readm(&title).await;

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("path/to/your/icon.ico");
        res.compile().expect("Failed to set icon");
    }
    
    Ok(())
}
