use reqwest::Error;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error as StdError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Helper function to unwrap
fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

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
    // Define multiple selectors for extracting columns (relevant cells from each row) 
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("table tr").unwrap();
    let header_selector = Selector::parse("th").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut lz_houston_data = Vec::new();
    let mut lz_houston_column_index: Option<usize> = None;

    // O(N^K) | Iterate over rows to extract values for LZ_HOUSTON
    for row in document.select(&row_selector) {
        // Iterate over header cells until reaching "LZ_HOUSTON"
        if lz_houston_column_index.is_none() {
            let mut header_index = 0;
            for header in row.select(&header_selector) {
                if header.text().collect::<String>().trim() == "LZ_HOUSTON" {
                    lz_houston_column_index = Some(header_index);
                    break;
                }
                header_index += 1;
            }
        } 
        
        // Iterate over each row to extract data under "LZ_HOUSTON" column
        else {
            let mut cell_index = 0;
            let mut lz_houston_value = None;

            // O(K) | Extract value from individual rows
            for cell in row.select(&cell_selector) {
                if cell_index == lz_houston_column_index.unwrap() {
                    lz_houston_value = Some(cell.text().collect::<String>());
                    break;
                }
                cell_index += 1;
            }

            if let Some(value) = lz_houston_value {
                lz_houston_data.push(Data { lz_houston: value });
            }
        }
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
