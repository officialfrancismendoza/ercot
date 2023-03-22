use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let response = reqwest::blocking::get(
        https://www.ercot.com/content/cdr/html/20230322_dam_spp.html
    )
    .unwrap()
    .text()
    .unwrap();
}
