use serde::{Deserialize, Serialize};
extern crate reqwest;
use chrono::{Duration, Local};
use colored::Colorize;
use reqwest::header;
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct Ad {
    id: String,
    published_date: Option<String>,
    last_application_date: Option<String>,
    title: String,
    occupation: String,
    workplace: Option<String>,
    workplace_name: Option<String>,
    unspecified_workplace: Option<String>,
    published: bool,
    positions: i32,
    source_links: Option<Vec<SourceLink>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SourceLink {
    label: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Jobs {
    positions: i32,
    numberOfAds: i32,
    offset_limit: Option<i32>,
    ads: Vec<Ad>,
}

fn deserialize_job(json_str: String) -> Jobs {
    match from_str(&json_str) {
        Ok(job) => job,
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            panic!();
        }
    }
}

pub fn platsbanken(keyword: String) {
    let client = get_client();

    let mut start_idx = 0;
    let mut counter = 0;

    'ads: loop {
        let json_body = get_json_body(&start_idx.to_string(), &keyword);
        let result = get_request(&client, json_body);

        for job in result.ads {
            let link_string = format!(
                "\x1B]8;;{}\x07{}\x1B]8;;\x07",
                "https://arbetsformedlingen.se/platsbanken/annonser/".to_owned()
                    + &job.id.to_string(),
                &job.title
            );
            println!("{}: {}", link_string.blue(), counter);
            counter += 1;
            //println!("{}\n{}", job.id, job.title);
        }
        if result.numberOfAds < start_idx + 100 {
            break 'ads;
        }
        start_idx += 100;
    }
}

fn get_json_body(start_index: &str, keyword: &String) -> String {
    let now = Local::now();
    let thirty_days_ago = now - Duration::days(30);

    let from_date = thirty_days_ago.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
    let to_date = now.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();

    let key_word = &keyword[..keyword.len() - 1];

    "{\"filters\":[{\"type\":\"freetext\",\"value\":\"key_word\"}],\"fromDate\":\"from_date\",\"order\":\"relevance\",\"maxRecords\":100,\"startIndex\":start_index,\"toDate\":\"to_date\",\"source\":\"pb\"}".replace("key_word", key_word).replace("start_index", start_index).replace("from_date", &from_date).replace("to_date", &to_date)
}

fn get_request(client: &reqwest::blocking::Client, json_body: String) -> Jobs {
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

    let response = client
        .post("https://platsbanken-api.arbetsformedlingen.se/jobs/v1/search")
        .headers(headers)
        .body(json_body)
        .send()
        .unwrap()
        .text()
        .unwrap();

    deserialize_job(response)
    //println!("{:?}", response);
    // println!("{:?}", response);
    // Ok(())
}

fn get_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}
