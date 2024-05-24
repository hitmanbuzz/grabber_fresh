use std::{fs::File, io::Write, path::Path, process::exit};
use std::{collections::HashSet, fs, io};
use tokio::task;


use reqwest::Client;

async fn fetch_comic_chapter(comic_url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

async fn fetch_chapter_url(chapter_url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

async fn fetch_comic_image(image_url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(image_url).send().await?.bytes().await?;
    let mut file = File::create(path)?;
    file.write_all(&response)?;
    Ok(())
}

pub async fn readm(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let title: Vec<&str> = url.split("/manga/").collect();
    let title = title[1];
    let mut response = fetch_comic_chapter(url.to_owned()).await.unwrap();
    let file_format = ".jpg";

    response.reverse();
    let set: HashSet<_> = response.into_iter().collect();
    let mut deduped_urls: Vec<_> = set.into_iter().collect();

    deduped_urls.sort_by(|a, b| {
        let num_a = a.split('/').nth(5).unwrap_or("0").parse::<usize>().unwrap_or(0);
        let num_b = b.split('/').nth(5).unwrap_or("0").parse::<usize>().unwrap_or(0);
        num_a.cmp(&num_b)
    });

    // println!("Sorted ALL URL: {:#?}", deduped_urls);

    let mut handles = vec![];
    let sub_string = format!("/manga/{}/", title);

    for u in deduped_urls {
        let chapter_number: Vec<&str> = u.split(&sub_string).collect();
        let chapter_number: Vec<&str> = chapter_number[1].split("/").collect();
        let r = fetch_chapter_url(u.clone()).await.unwrap();
        for (index, url) in r.iter().enumerate() {
            let url = url.to_string();
            println!("Chapter Number: {}", chapter_number[0]);
            let folder = format!("download\\{}\\chapter_{}", title, chapter_number[0]);
            let _ = fs::create_dir_all(folder);

            let path = format!("download\\{}\\chapter_{}\\image{}{}", title, chapter_number[0], index, file_format);
            let path = Path::new(&path).to_path_buf();
            let handle = task::spawn(async move {
                fetch_comic_image(&url, &path).await.unwrap();
            });
            handles.push(handle);
        }
        println!("\n");
    }

    for handle in handles {
        handle.await?;
    }

    println!("Done!!!");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("failed to read str");

    Ok(())
}