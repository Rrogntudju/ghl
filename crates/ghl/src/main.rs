use regex::Regex;
use reqwest::blocking::Client;
use soup::prelude::*;
use std::env::args;
use std::error::Error;
use url::Url;
fn main() -> Result<(), Box<dyn Error>> {
    if let Some(url) = args().nth(1) {
        let page = Client::new().get(Url::parse(&url)?).send()?.text()?;
        let soup = Soup::new(&page);
        match soup.tag("title").find() {
            Some(titre) => {
                let regex = Regex::new(r"^Release (.*) · cloudflare/cloudflared · GitHub$")?;
                if let Some(captures) = regex.captures(&titre.text()) {
                    println!("{}", captures.get(1).unwrap().as_str());
                    Ok(())
                } else {
                    Err("version introuvable".into())
                }
            }
            None => Err("titre introuvable".into()),
        }
    } else {
        Err("Url «latest» manquant".into())
    }
}
