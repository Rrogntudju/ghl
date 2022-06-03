use regex::{escape, Regex};
use reqwest::blocking::Client;
use soup::prelude::*;
use std::env::args;
use std::error::Error;
use url::Url;
fn main() -> Result<(), Box<dyn Error>> {
    let url_latest = match args().nth(1) {
        Some(url) => Url::parse(&url)?,
        None => return Err("Url «latest» manquant".into()),
    };
    let download = match args().nth(2) {
        Some(download) => download,
        None => return Err("Fichier à télécharger manquant".into()),
    };
    let page = Client::new().get(url_latest).send()?.text()?;
    let soup = Soup::new(&page);
    let regex = format!(r"^.+/{}$", escape(&download));
    match soup.tag("a").attr("href", Regex::new(&regex)?).find() {
        Some(tag) => println!("https://github.com{}", tag.get("href").unwrap()),
        None => return Err("Fichier à télécharger introuvable".into()),
    };
    Ok(())
}
