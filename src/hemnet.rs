pub fn hemnet() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get("https://www.hemnet.se/bostader/search/50162216c078bfead5b022e473b4b003e80d4105")
        //.headers(headers)
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
