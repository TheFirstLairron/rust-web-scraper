extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main() {
    let url = "https://www.google.com";
    let document = get_html(url);
    match document {
        Ok(doc) => {
            let selector = Selector::parse("a").unwrap();
            let html = Html::parse_document(&doc[..]);
            println!("Links ({}): ", url);
            for element in html.select(&selector) {
                let new_url = match element.value().attr("href") {
                    Some(link) => link,
                    None => "No URL associated with this link",
                };
                let text = element.text().collect::<Vec<_>>().join("");
                println!("{}:   {}", text, new_url);
            }
        }
        _ => {
            println!("Failed to parse html");
        }
    }
}

fn get_html(url: &str) -> Result<String, ()> {
    let result = reqwest::get(url);
    match result {
        Ok(mut response) => match response.text() {
            Ok(content) => Ok(content),
            _ => Err(()),
        },
        _ => Err(()),
    }
}
