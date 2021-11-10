use regex::Regex;
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
    let page = minreq::get(url_latest.as_str()).with_timeout(10).send()?;
    let soup = Soup::new(page.as_str()?);
    let regex = Regex::new(&format!(r"^.*?/{}$", download.replace(".", r"\.")))?;
    match soup.tag("a").attr("href", regex).find() {
        Some(tag) => println!("https://github.com{}", tag.get("href").unwrap()),
        None => return Err("Fichier à télécharger introuvable".into()),
    };
    Ok(())
}
