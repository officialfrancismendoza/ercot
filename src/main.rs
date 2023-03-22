use reqwest::Error;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error as StdError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Struct containing all the columns to scrape 
#[derive(Debug, Serialize)]
struct Data {
    lz_houston: String,
    lz_north: String,
    lz_south: String,
    lz_west: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let url = "https://www.ercot.com/content/cdr/html/20230322_dam_spp.html";
    let html_data = fetch_html(url).await?;

    let combined_data = extract_data(&html_data)?;
    save_to_csv(combined_data)?;
    println!("Data saved to combined.csv");
    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn extract_data(html: &str) -> Result<Vec<Data>, Box<dyn StdError>> {
    // Relevant selectors to cumulatively process elements from select columns
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("table tr").unwrap();
    let header_selector = Selector::parse("th").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut data = Vec::new();
    let mut column_indices: Option<(usize, usize, usize, usize)> = None;
    let column_names = ["LZ_HOUSTON", "LZ_NORTH", "LZ_SOUTH", "LZ_WEST"];

    for row in document.select(&row_selector) {
        if column_indices.is_none() {
            let mut header_indices = (None, None, None, None);
            let mut header_index = 0;

            // Iterate through first row and extract target column names
            for header in row.select(&header_selector) {
                let header_text_raw = header.text().collect::<String>();
                let header_text = header_text_raw.trim().to_owned();

                if header_text == column_names[0] {
                    header_indices.0 = Some(header_index);
                } else if header_text == column_names[1] {
                    header_indices.1 = Some(header_index);
                } else if header_text == column_names[2] {
                    header_indices.2 = Some(header_index);
                } else if header_text == column_names[3] {
                    header_indices.3 = Some(header_index);
                }

                if header_indices.0.is_some()
                    && header_indices.1.is_some()
                    && header_indices.2.is_some()
                    && header_indices.3.is_some()
                {
                    break;
                }

                header_index += 1;
            }

            column_indices = Some((
                header_indices.0.unwrap(),
                header_indices.1.unwrap(),
                header_indices.2.unwrap(),
                header_indices.3.unwrap(),
            ));
        } else {
            let column_indices = column_indices.unwrap();
            let mut row_data = Data {
                lz_houston: String::new(),
                lz_north: String::new(),
                lz_south: String::new(),
                lz_west: String::new(),
            };

            // Extract data from each column
            let mut cell_index = 0;
            for cell in row.select(&cell_selector) {
                if cell_index == column_indices.0 {
                    row_data.lz_houston = cell.text().collect::<String>();
                } else if cell_index == column_indices.1 {
                    row_data.lz_north = cell.text().collect::<String>();
                } else if cell_index == column_indices.2 {
                    row_data.lz_south = cell.text().collect::<String>();
                } else if cell_index == column_indices.3 {
                    row_data.lz_west = cell.text().collect::<String>();
                }

                cell_index += 1;
            }

            data.push(row_data);
        }
    }

    Ok(data)
}

fn save_to_csv(data: Vec<Data>) -> Result<(), Box<dyn StdError>> {
    let path = Path::new("combined.csv");
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    for record in data {
        csv_writer.serialize(record)?;
    }

    let csv_data = String::from_utf8(csv_writer.into_inner()?)?;
    let mut file = File::create(&path)?;
    file.write_all(csv_data.as_bytes())?;

    Ok(())
}
