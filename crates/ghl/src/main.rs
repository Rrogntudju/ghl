use regex::Regex;
use reqwest::blocking::Client;
use soup::prelude::*;
use std::error::Error;

const LATEST: &str = "https://github.com/cloudflare/cloudflared/releases/latest";
const REGEX: &str = r"^Release (.*) · cloudflare/cloudflared · GitHub$";

fn main() -> Result<(), Box<dyn Error>> {
    let page = Client::new().get(LATEST).send()?.text()?;
    let soup = Soup::new(&page);
    match soup.tag("title").find() {
        Some(titre) => {
            let regex = Regex::new(REGEX)?;
            if let Some(captures) = regex.captures(&titre.text()) {
                println!("{}", captures.get(1).unwrap().as_str());
                Ok(())
            } else {
                Err("version introuvable".into())
            }
        }
        None => Err("titre introuvable".into()),
    }
}
