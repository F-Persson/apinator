use serde::{Deserialize, Serialize};
extern crate reqwest;
use chrono::{Duration, Local};
use colored::Colorize;
use reqwest::header;
use serde_json::{from_str, Value};

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

pub fn platsbanken_twist(keyword: String, includes: Vec<String>) {
    let client = get_client();

    let mut start_idx = 0;
    let mut counter = 0;
    let mut job_url: Vec<String> = vec![];

    //let mut jobs: Vec<Jobs> = vec![];

    'ads: loop {
        let json_body = get_json_body(&start_idx.to_string(), &keyword);
        let result = get_request(&client, json_body);

        // This is to get every single job Id
        // for job in result.ads {
        //     job_url.push(
        //         "https://platsbanken-api.arbetsformedlingen.se/jobs/v1/job/".to_owned()
        //             + &job.id.to_string(),
        //     );
        // }

        //let job_url =
        // jobs.push(result);

        // Prints the url to the job
        for job in result.ads {
            job_url.push(
                "https://platsbanken-api.arbetsformedlingen.se/jobs/v1/job/".to_owned()
                    + &job.id.to_string(),
            );
            //println!("{}\n{}", job.id, job.title);
        }
        if result.numberOfAds < start_idx + 100 {
            break 'ads;
        }
        start_idx += 100;
    }

    // Get and print every single job
    get_jobs(&client, &job_url, &includes);
}

fn get_jobs(
    client: &reqwest::blocking::Client,
    job_url: &[String],
    includes: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    for str in job_url {
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
        headers.insert("INT_SYS", "platsbanken_web_beta".parse().unwrap());
        headers.insert("Origin", "https://arbetsformedlingen.se".parse().unwrap());
        headers.insert("Referer", "https://arbetsformedlingen.se/".parse().unwrap());
        let response = client
            .get(str)
            .headers(headers)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let json_value: Value = serde_json::from_str(&response)?;

        for include in includes {
            if json_value["description"]
                .to_string()
                .to_ascii_lowercase()
                .contains(&*include.to_ascii_lowercase())
            {
                let link_string = format!(
                    "\x1B]8;;{}\x07{}\x1B]8;;\x07",
                    "https://arbetsformedlingen.se/platsbanken/annonser/".to_owned()
                        + &json_value["id"].to_string().replace("\"", ""),
                    json_value["title"].to_string()
                );
                println!("{}", link_string.blue());
                break; // break out of the loop when the first search string is found
            }
        }

        //println!("{}", json_value["description"]);
        //println!("{response}");
    }

    Ok(())
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
