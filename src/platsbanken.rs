use serde::{Deserialize, Serialize};
extern crate reqwest;
use chrono::{Duration, Local};
use reqwest::header;
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
struct Ad {
    id: String,
    published_date: String,
    last_application_date: Option<String>,
    title: String,
    occupation: String,
    workplace: Option<String>,
    workplace_name: String,
    unspecified_workplace: Option<String>,
    published: bool,
    positions: i32,
    source_links: Vec<SourceLink>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SourceLink {
    label: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Root {
    positions: i32,
    number_of_ads: i32,
    offset_limit: i32,
    ads: Vec<Ad>,
}

pub fn platsbanken(keyword: String) {
    let now = Local::now();
    let thirty_days_ago = now - Duration::days(30);

    let from_date = thirty_days_ago.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
    let to_date = now.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();

    let key_word = &keyword[..keyword.len() - 1];

    let json_body = "{\"filters\":[{\"type\":\"freetext\",\"value\":\"key_word\"}],\"fromDate\":\"2022-12-21T23:00:00.000Z\",\"order\":\"relevance\",\"maxRecords\":100,\"startIndex\":0,\"toDate\":\"2023-01-20T06:41:22.487Z\",\"source\":\"pb\"}".replace("key_word", key_word).replace("from_date", &from_date).replace("to_date", &to_date);

    let result = get_request(json_body);

    println!("{:?}", result);
}

fn get_request(json_body: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert(
        "Accept-Language",
        "en-US,en;q=0.9,sv;q=0.8".parse().unwrap(),
    );
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .post("https://platsbanken-api.arbetsformedlingen.se/jobs/v1/search")
        .headers(headers)
        .body(json_body)
        .send()?
        .text()?;
    println!("{}", res);
    Ok(())
}

// for id in res {
//      let url: String =
//          "https://arbetsformedlingen.se/platsbanken/annonser/".to_owned() + &id.to_string();
//     let res = client.get(i).send()?.text()?;
//     let response = deserialize_hackernews_story(res);
//     //println!("{}\n", response.url?);
//     let link_string = format!(
//         "\x1B]8;;{}\x07{}\x1B]8;;\x07",
//         &response.url.unwrap_or("default_url".to_string()),
//         &response.title
//     );
//     println!("{}", link_string.red());
// }

//
