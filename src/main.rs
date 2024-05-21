mod source;
use std::{collections::HashSet, fs, io, path::Path};
use tokio::task;

use crate::source::readm::{fetch_comic_chapter, fetch_chapter_url, fetch_comic_image};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://readm.today/manga/18144");
    let title: Vec<&str> = url.split("/manga/").collect();
    let title = title[1];
    let mut response = fetch_comic_chapter(url.clone()).await.unwrap();
    let mut file_format = ".jpg";

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
        println!("LVL 0 Reached");
        let chapter_number: Vec<&str> = u.split(&sub_string).collect();
        let chapter_number: Vec<&str> = chapter_number[1].split("/").collect();
        let r = fetch_chapter_url(u.clone()).await.unwrap();
        for (index, url) in r.iter().enumerate() {
            println!("LVL 0.5 Reached");
            let url = url.to_string();
            println!("Chapter Number: {}", chapter_number[0]);
            let folder = format!("download\\{}\\chapter_{}", title, chapter_number[0]);
            let _ = fs::create_dir_all(folder);
            if !url.contains(".jpg") {
                file_format = ".png";
            }

            println!("LVL 2 Reached");
            let path = format!("download\\{}\\chapter_{}\\image{}{}", title, chapter_number[0], index, file_format);
            let path = Path::new(&path).to_path_buf();
            let handle = task::spawn(async move {
            println!("LVL 3 Reached");
                fetch_comic_image(&url, &path).await.unwrap();
            });
            handles.push(handle);
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
