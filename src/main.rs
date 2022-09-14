use error_chain::error_chain;
use serde_derive::*;
use std::{ io::Read};
use rand::Rng;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize)]
//template for json response
struct Character {
    name: String,
    status: String,
    gender: String,
    origin: Origin,
}

#[derive(Serialize, Deserialize)]
struct Origin {
    name: String,
    url: String,
}

fn main() {
    println!("getting response...");
    get_character_from_api().expect("An error occoured");
    println!("Done!");
}

fn get_character_from_api() -> Result<()> {
    let mut random = rand::thread_rng();

    let random_character: u8 = random.gen();
   
    let url = format!("https://rickandmortyapi.com/api/character/{}", random_character);
    
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();

    // read_to_string() method which stores the response
    res.read_to_string(&mut body).expect("Couldn't read from str");

    println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let parsed_body = serde_json::from_str(&body);

    if parsed_body.is_ok() {
        let p: Character = parsed_body.unwrap();

        println!("
        The name is {}, status: {}. Location: {}.
        ",
            p.name, p.status, p.origin.name
        );
    } else {
        println!("error could not parse json....")
    }

    Ok(())
}
