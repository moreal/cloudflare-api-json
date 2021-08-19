use soup::prelude::*;

const CLOUDFLARE_API_URL: &str = "https://api.cloudflare.com/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(CLOUDFLARE_API_URL)?;
    let soup = Soup::from_reader(response)?;
    for el in soup.class("modunit") {
        let anchor2 = el.class("anchor2").find();
        let small = el.tag("div").class("mod-header").find()
            .and_then(|e|
                e.tag("h3").class("mod-title").find()
            ).and_then(|e|
                e.tag("small").find()
            );
        let language_http = el.tag("pre").class("language-http").find();
        let label_info = el.tag("div").class("mod-header").find()
            .and_then(|e|
                e.tag("h3").class("mod-title").find()
            ).and_then(|e|
                e.class("label-info").find()
            );

        if anchor2.is_none() || small.is_none() || label_info.is_none() || language_http.is_none() {
            continue;
        }

        let anchor2 = anchor2.unwrap();

        let small = small.unwrap();
        let permissions = small.text().replace("permission needed: ", "");

        let language_http = language_http.unwrap();
        let api_description = language_http.text();
        let splitted = api_description.split(' ').collect::<Vec<&str>>();
        let method = splitted[0];
        let url = splitted[1];

        println!("{}", anchor2.text());
        println!("{}", permissions);
        println!("{}", method);
        println!("{}", url);

        println!("");
    }

    Ok(())
}