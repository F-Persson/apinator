extern crate reqwest;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct HackernewsStruct {
    by: String,
    descendants: Option<i32>,
    id: i32,
    kids: Option<Vec<i32>>,
    score: i32,
    time: i32,
    title: String,
    type_field: Option<String>,
    url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct HackernewsTopstoriesStruct(Vec<i32>);

fn deserialize_hackernews_top_stories(json_str: String) -> HackernewsTopstoriesStruct {
    match from_str(&json_str) {
        Ok(job) => job,
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            panic!();
        }
    }
}

fn deserialize_hackernews_story(json_str: String) -> HackernewsStruct {
    match from_str(&json_str) {
        Ok(job) => job,
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            panic!();
        }
    }
}

pub fn hacker_news() {
    let client = get_client();

    let story_urls = get_all_id(&client);

    get_all_stories(client, story_urls);
}

fn get_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

fn get_all_stories(client: reqwest::blocking::Client, story_urls: Vec<String>) {
    for url in story_urls {
        let response = client.get(url).send().unwrap().text().unwrap();

        let response = deserialize_hackernews_story(response);

        let link_string = format!(
            "\x1B]8;;{}\x07{}\x1B]8;;\x07",
            &response.url.unwrap_or("default_url".to_string()),
            &response.title
        );
        println!("{}", link_string.red());
    }
}

fn get_all_id(client: &reqwest::blocking::Client) -> Vec<String> {
    let response = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let response = deserialize_hackernews_top_stories(response);

    response
        .0
        .into_iter()
        .map(|x| {
            "https://hacker-news.firebaseio.com/v0/item/".to_owned() + &x.to_string() + ".json"
        })
        .collect()
}

//chat-gpt recommends:
// impl<T> T where T: serde::de::DeserializeOwned {
//     fn from_json(json_str: String) -> Self {
//         match serde_json::from_str(&json_str) {
//             Ok(job) => job,
//             Err(e) => {
//                 println!("Failed to parse JSON: {}", e);
//                 panic!();
//             }
//         }
//     }
// }

// let top_stories = HackernewsTopstoriesStruct::from_json(json_string);
// let story = HackernewsStruct::from_json(json_string);
