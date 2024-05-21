mod source;

use std::{collections::HashSet, fs, io, path::Path};

use regex::Regex;
use tokio::task;

use crate::source::readm::{fetch_comic_chapter, fetch_chapter_url, fetch_comic_image};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://readm.today/manga/my-furry-harem-is-after-me/");
    let title: Vec<&str> = url.split("/manga/").collect();
    let title = title[1];
    let re_format = format!(r"/{}/(\d+)/", title);
    let re = Regex::new(&re_format).unwrap();
    let mut response = fetch_comic_chapter(url.clone()).await.unwrap();

    response.reverse();
    let set: HashSet<_> = response.into_iter().collect();
    let mut deduped_urls: Vec<_> = set.into_iter().collect();

    deduped_urls.sort_by(|a, b| {
        let num_a = a.split('/').nth(5).unwrap_or("0").parse::<usize>().unwrap_or(0);
        let num_b = b.split('/').nth(5).unwrap_or("0").parse::<usize>().unwrap_or(0);
        num_a.cmp(&num_b)
    });

    println!("Sorted ALL URL: {:#?}", deduped_urls);

    let mut handles = vec![];

    for u in deduped_urls {
        let r = fetch_chapter_url(u).await.unwrap();
        for (index, url) in r.iter().enumerate() {
            let url = url.to_string();
            if let Some(caps) = re.captures(&url) {
                if let Some(matched) = caps.get(1) {
                    println!("Chapter Number: {}", matched.as_str());
                    let folder = format!("download\\{}\\chapter_{}", title, matched.as_str());
                    let _ = fs::create_dir_all(folder);
                    let path = format!("download\\{}\\chapter_{}\\image{}.jpg", title, matched.as_str(), index);
                    let path = Path::new(&path).to_path_buf();
                    let handle = task::spawn(async move {
                        fetch_comic_image(&url, &path).await.unwrap();
                    });
                    handles.push(handle);
                }
            }
            
        }
    }

    for handle in handles {
        handle.await?;
    }

    println!("Done!!!");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("failed to read str");

    Ok(())
}
