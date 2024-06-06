use reqwest::Client;
use std::thread::sleep;
use std::time::Duration;
use std::{collections::HashSet, fs};
use std::{fs::File, io::Write, path::Path, process::exit};
use tokio::task;

/// Fetch comic chapters url
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

/// Fetch the comic each chapters images url available
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

/// Download Images
async fn fetch_comic_image(image_url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(image_url).send().await?.bytes().await?;
    let mut file = File::create(path)?;
    file.write_all(&response)?;
    Ok(())
}

/// READM Function
pub async fn readm(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let title_parts: Vec<&str> = url.split("/manga/").collect();
    let title = title_parts[1].to_string(); // Ensure `title` is owned
    let mut response = fetch_comic_chapter(url.to_owned()).await?;
    let file_format = ".jpg";

    response.reverse();
    let set: HashSet<_> = response.into_iter().collect();
    let mut deduped_urls: Vec<_> = set.into_iter().collect();

    deduped_urls.sort_by(|a, b| {
        let num_a = a
            .split('/')
            .nth(5)
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let num_b = b
            .split('/')
            .nth(5)
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        num_a.cmp(&num_b)
    });

    // let mut handles = vec![];
    let sub_string = format!("/manga/{}/", title);
    let timer = Duration::from_millis(1000);

    for u in deduped_urls {
        let chapter_number_parts: Vec<&str> = u.split(&sub_string).collect();
        let chapter_number_parts: Vec<&str> = chapter_number_parts[1].split('/').collect();
        let chapter = chapter_number_parts[0].to_string(); // Ensure `chapter` is owned
        let r = fetch_chapter_url(u.clone()).await?;
        for (index, url) in r.iter().enumerate() {
            let url = url.to_string(); // Ensure `url` is owned
            let folder = format!("download/{}\\chapter_{}", title, chapter);
            if let Err(err) = fs::create_dir_all(&folder) {
                eprintln!("Failed to create directory {}: {}", folder, err);
                continue;
            }

            let path = format!(
                "download/{}\\chapter_{}\\image{}{}",
                title, chapter, index, file_format
            );
            let path = Path::new(&path).to_path_buf();

            // Clone necessary variables to move into async block
            let chapter_clone = chapter.clone();
            let path_clone = path.clone();
            let url_clone = url.clone();
            let timer_clone = timer.clone();

            let _ = task::spawn(async move {
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
            // handles.push(handle);
        }
    }

    // for handle in handles {
    //     handle.await?;
    // }

    Ok(())
}
