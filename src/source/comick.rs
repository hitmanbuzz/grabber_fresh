use core::time;
use std::{fs::{self, File}, io::{Read, Write}, path::Path, process::exit, thread::sleep};
use reqwest::Client;
use serde_json::Value;
use serde::Deserialize;
use tokio::task;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Chapter {
    chap: String,
    hid: String,
    vol: Option<Value>,
    lang: String,
    id: u64,
    title: Option<Value>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Images {
    images: Vec<String>,
    safe_title: String,
}

pub async fn fetch_chapter1(comic_url: String) -> Result<String, Box<dyn std::error::Error>> {
    let first_substring = "firstChap";
    let client = reqwest::Client::builder().default_headers({
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, "PostmanRuntime/7.38.0".parse().unwrap());
        headers
    })
        .build()?;
    let request = client.get(comic_url).send().await?;
    if !request.status().is_success() {
        println!("Http Request Error At Fetching Comic Chapter 1");
        exit(0);
    }
    let target = "hid";

    let response = request.text().await?;

    for i in response.lines() {
        if i.contains(first_substring) {
            let index: Vec<&str> = i.split(first_substring).collect();
            let first_index: Vec<&str> = index[1].split("},").collect();
            let first_index = first_index[0];

            if !first_index.contains("hid") {
                println!("Does't contain hid substring");
            }

            let next_index: Vec<&str> = first_index.split(target).collect();
            let next_index = next_index[1];

            let next_index: Vec<&str> = next_index.split(":").collect();
            let next_index: Vec<&str> = next_index[1].split(",").collect();
            let chapter = next_index[0].replace('"', "");
            return Ok(chapter);
        }
    }

    return Ok("Chapter 1 HID: Not Found".to_owned());
}

pub async fn extract_data(chapter1_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().default_headers({
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, "PostmanRuntime/7.38.0".parse().unwrap());
        headers
    })
        .build()?;
    let target = "chapters\":[";
    let request = client.get(chapter1_url).send().await?;

    let response = request.text().await?;

    for i in response.lines() {
        if i.contains(target) {
            let index: Vec<&str> = i.split(target).collect();
            let index: Vec<&str> = index[1].split(']').collect();
            let index = index[0];
            let mut file = File::create("result.json")?;
            let data_format: String = format!("[{}]", index);
            let _ = file.write_all(data_format.as_bytes());
            println!("Check response.json!!!")
        }
    }

    Ok(())
    
}
async fn fetch_comic_image(image_url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(image_url).send().await?.bytes().await?;
    let mut file = File::create(path)?;
    file.write_all(&response)?;
    Ok(())
}

pub async fn fetch_comic_chapter(comic_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().default_headers({
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, "PostmanRuntime/7.38.0".parse().unwrap());
        headers.insert(reqwest::header::COOKIE, "_ga=GA1.1.389011008.1713623107; csrf_token_88f5949651c397a49661b0911c395e7a15e6eb27750c2b2165bf0bfea86cc369=YvF2xQUKJDdtvVdiSsI59DNST5oPQ9P33JEubjF9aaI=; ory_kratos_session=MTcxMzYzMzI1NnxFMGNXN2p1OHRaV2t1QTBMaTlZazJQZ3hjVUo4c3hoOF85QTFPME9EQ0F4eU9sZ0x4NlBlQy1lQ3lHNVV5b19TcW45VEtJY0ZTcXJzUlA5TWZuMG5YT0pkb09YT2NiSENDSldwc3FLa1RwMkkySmo5SEgyQk85eDFuRjBSOVA4U3Q3SGJRZ1c3OWhZYU9IOEZ4el95clRuOFZpd1ByNDJtSzNVWWd1T29qN3cwY2otTGd0OFpkWWZ4Znc4Y3BhY19rOVlodW5TT2JTeVlGbDBDSjdOckczaDdJWnJray1ubkxHdDlwd1JRSWlOV3pMTFNORXc1UW5yVUVTSDN4dm9xamJNSzNYczFnZmpnUVgwcU9XZG98HwKpqHCQC_k_-UWA-YmDC6OsCO6JlFrhHjdqkwERmog=; cf_clearance=9RpAXF7HUt8927VyYkjYszxIbulfjLzpGcK5o0gZVAg-1715174106-1.0.1.1-cRd6YErIQ6Eg1Y9pYIt3HPRzAej_XOma4UVp99im6avWbo1kexvlO_7nMIDUmzN5f1Xx82ddKeIBm1qKxrLvnA; cf_clearance=jb11qOKZfmPLsev.d0w4KnEkpnmLnXNDkqXWRYzQ_qE-1716474769-1.0.1.1-d1KfRaYSbt2CZOfmZanC7t2nRIYvb4wA2UP_knfZT.EQeM9lzESMG.Dw_4IuxIpgCg8MnWTlt3vr0lxNOH609A; _ga_E39DR6FRXE=GS1.1.1716474770.7.0.1716474770.0.0.0".parse().unwrap());
        headers
    })
        .build()?;

    let mut file = File::open("result.json").expect("Failed to open file");

    // Read the file contents into a string
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    // Deserialize the JSON data
    let chapters: Vec<Chapter> = serde_json::from_str(&json_data).unwrap();

    let mut handles = vec![];
    let file_format = ".jpg";
    let timer = time::Duration::from_millis(1000);
    // Access the `chap` and `hid` values
    for chapter in chapters {
        let url = format!("https://api.comick.io/v1.0/chapter/{}/download", chapter.hid.trim());
        let result = client.get(url).send().await?;
        if !result.status().is_success() {
            println!("{}", result.status());
            println!("Http Request Error At Fetching Comic Chapters");
            exit(0);
        }
        let response = result.text().await?;
        let images_data: Images = serde_json::from_str(&response).unwrap();
        for (index, image) in images_data.images.iter().enumerate() {
            let chapter = chapter.chap.clone();
            let folder = format!("download\\{}\\chapter_{}", comic_title, chapter);
            let _ = fs::create_dir_all(folder);
            let path = format!("download\\{}\\chapter_{}\\image{}{}", comic_title, chapter, index, file_format);
            let path = Path::new(&path).to_path_buf();
            let image_url = format!("https://meo3.comick.pictures/{}", image);
            let handle = task::spawn(async move {
                match fetch_comic_image(&image_url, &path).await {
                    Ok(_) => {
                        println!("[Chapter: {} | Image: {}] Download Finished\n", chapter, index);
                    }
                    Err(_) => {
                        sleep(timer);
                    },
                }
            });
            handles.push(handle);
            
        }
    }
    
    for handle in handles {
        handle.await?;
    }

    println!("Done!!!");

    Ok(())
}

pub async fn comick(comic_url: String) {
    let comic_title: &Vec<&str> = &comic_url.split("comic/").collect();
    let comic_title = comic_title[1];
    let chapter1_url = format!("{}/{}-chapter-1-en", &comic_url, fetch_chapter1(comic_url.clone()).await.unwrap());
    extract_data(chapter1_url).await.unwrap();
    fetch_comic_chapter(comic_title).await.unwrap();
}