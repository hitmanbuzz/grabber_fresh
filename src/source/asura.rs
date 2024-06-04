use reqwest::Client;
use scraper::{Html, Selector};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
};
use tokio::task;

pub async fn asura() {
    let mut handles = vec![];
    let url = "https://asuratoon.com/manga/my-exclusive-tower-guide/";
    let title: Vec<&str> = url.split("manga/").collect();
    // Title = title[0]
    let title: Vec<&str> = title[1].split("/").collect();
    let chapter_url = fetch_chapter(url).await.unwrap();
    for (i, u) in chapter_url.iter().enumerate() {
        let chapter: Vec<&str> = u.split("chapter-").collect();
        //  = title[0]
        let chapter: Vec<&str> = chapter[1].split("/").collect();
        let dir = format!("download/{}/chapter_{}", title[0], chapter[0]);
        if let Err(err) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory {}: {}", dir, err);
            continue;
        }

        let path = format!(
            "download/{}/chapter_{}/image{}.jpg",
            title[0], chapter[0], i
        );
        let path = Path::new(&path).to_path_buf();

        let handle = task::spawn(async move {
            match fetch_comic_image(&url_clone, &path_clone).await {
                Ok(_) => {
                    println!(
                        "[Chapter: {}| Image: {} ] Download Finished\n",
                        chapter_clone, index
                    );
                }
                Err(_) => {
                    sleep(timer_clone);
                }
            }
        });
        handles.push(handle);
    }
}

pub async fn fetch_image(comic_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut container = Vec::new();
    let client = reqwest::Client::builder().build()?;
    let request = client.get(comic_url).send().await?;
    if !request.status().is_success() {
        println!("Http Request Error fetching comic chapter");
        exit(0);
    }

    let response = request.text().await?;
    let document = Html::parse_document(&response);

    // Define the selector for the class element
    let selector = Selector::parse(".ts-main-image").unwrap();

    // Iterate over the selected elements and print the `src` attribute
    for element in document.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            container.push(src.to_string());
        }
    }

    Ok(container)
}

async fn fetch_chapter(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut url_container = Vec::new();
    let client = reqwest::Client::builder().build()?;
    let request = client.get(url).send().await?;
    if !request.status().is_success() {
        println!("Http Request Error fetching comic chapter");
        exit(0);
    }

    let response = request.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse("div.chbox a").unwrap();

    // Iterate over the selected elements and print the `href` attribute
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            url_container.push(href.to_string());
        }
    }

    url_container.reverse();

    Ok(url_container)
}

async fn fetch_comic_image(image_url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(image_url).send().await?.bytes().await?;
    let mut file = File::create(path)?;
    file.write_all(&response)?;
    Ok(())
}