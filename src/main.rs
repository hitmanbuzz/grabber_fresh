mod source;
use std::io::{self, stdout, Write};

use crate::source::asura::asura;
use crate::source::comick::comick;
use crate::source::readm::readm;

#[tokio::main]
async fn main() {
    println!("Grabber Made By Hitman|Moirangthem Henthoiba\n");
    print!(
        "[1] Readm.today
[2] Comick.io
[3] AsuraScans

[#] Choose: "
    );
    stdout().flush().unwrap();
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim();
    if option == "1" {
        print!("\nEnter URL: ");
        stdout().flush().unwrap();
        let mut readm_url = String::new();
        io::stdin().read_line(&mut readm_url).unwrap();
        let readm_url = readm_url.trim();
        readm(readm_url).await.unwrap();
    } else if option == "2" {
        print!("\nEnter URL: ");
        stdout().flush().unwrap();
        let mut comick_url = String::new();
        io::stdin().read_line(&mut comick_url).unwrap();
        let comick_url = comick_url.trim();
        comick(comick_url.to_string()).await;
    } else if option == "3" {
        print!("\nEnter URL: ");
        stdout().flush().unwrap();
        let mut asura_url = String::new();
        io::stdin().read_line(&mut asura_url).unwrap();
        asura(asura_url).await.unwrap();
    } else {
        println!("Wrong Option");
    }

    println!("\nFinished Downloading");
    println!("\nPress any key to exit...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    println!("You pressed: {}", buffer.trim());
}
