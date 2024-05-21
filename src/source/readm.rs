use std::{fs::File, io::Write, path::Path, process::exit};

use reqwest::Client;

pub async fn fetch_comic_chapter(comic_url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let request = client.get(comic_url).send().await?;
    let target_string = "/manga/";
    let mut container = Vec::new();

    if !request.status().is_success() {
        println!("Http Request Error At Fetching Comic Chapter");
        exit(0);
    }

    let response = request.text().await?;
    for i in response.lines() {
        if i.contains(target_string) {
            let sub_string: Vec<&str> = i.split(target_string).collect();
            let next_index: Vec<&str> = sub_string[1].split('"').collect();
            if next_index[0].contains("all-pages") {
                let result = format!("https://readm.today{}{}", target_string, next_index[0]);
                container.push(result);
            }
        }
    }

    Ok(container)
}

pub async fn fetch_chapter_url(chapter_url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let request = client.get(chapter_url).send().await?;
    let target_string = "?v=12";
    let mut container = Vec::new();

    if !request.status().is_success() {
        println!("Http Request Error At Fetching Chapter URL");
        exit(0);
    }

    let response = request.text().await?;
    for i in response.lines() {
        if i.contains(target_string) {
            let sub_string: Vec<&str> = i.split(target_string).collect();
            let previous_index: Vec<&str> = sub_string[0].split('"').collect();
            if previous_index[1].contains("/uploads") {
                let result = format!("https://readm.today{}", previous_index[1]);
                container.push(result)
            }
        }
    }

    Ok(container)
}

pub async fn fetch_comic_image(image_url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(image_url).send().await?.bytes().await?;
    let mut file = File::create(path)?;
    file.write_all(&response)?;
    Ok(())
}