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

pub fn hacker_news() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()?
        .text()?;
    //println!("{}", res);
    let response = deserialize_hackernews_top_stories(res);
    //let num: hackernews_topstories_struct = serde_json::from_str(&res).unwrap();
    //println!("{:?}", response.0[5]);

    /// Testing this
    let res: Vec<String> = response
        .0
        .into_iter()
        .map(|x| {
            "https://hacker-news.firebaseio.com/v0/item/".to_owned() + &x.to_string() + ".json"
        })
        .collect();

    /// End of testing here
    /// The below code works tho
    for i in res {
        // let url: String =
        //     "https://hacker-news.firebaseio.com/v0/item/".to_owned() + &i.to_string() + ".json";
        let res = client.get(i).send()?.text()?;
        let response = deserialize_hackernews_story(res);
        //println!("{}\n", response.url?);
        let link_string = format!(
            "\x1B]8;;{}\x07{}\x1B]8;;\x07",
            &response.url.unwrap_or("default_url".to_string()),
            &response.title
        );
        println!("{}", link_string.red());
    }
    Ok(())
}
