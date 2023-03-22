use reqwest::Error;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error as StdError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize)]
struct Data {
    lz_houston: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let url = "https://www.ercot.com/content/cdr/html/20230322_dam_spp.html";
    let html_data = fetch_html(url).await?;
    let lz_houston_data = extract_lz_houston(&html_data)?;
    save_to_csv(lz_houston_data)?;
    println!("Data saved to lz_houston.csv");
    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn extract_lz_houston(html: &str) -> Result<Vec<Data>, Box<dyn StdError>> {
    let document = Html::parse_document(html);
    let lz_houston_selector = Selector::parse("LZ_HOUSTON").unwrap();
    let mut lz_houston_data = Vec::new();

    for element in document.select(&lz_houston_selector) {
        let lz_houston = element.text().collect::<String>();
        lz_houston_data.push(Data { lz_houston });
    }

    Ok(lz_houston_data)
}

fn save_to_csv(data: Vec<Data>) -> Result<(), Box<dyn StdError>> {
    let path = Path::new("lz_houston.csv");
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    for record in data {
        csv_writer.serialize(record)?;
    }

    let csv_data = String::from_utf8(csv_writer.into_inner()?)?;
    let mut file = File::create(&path)?;
    file.write_all(csv_data.as_bytes())?;

    Ok(())
}
