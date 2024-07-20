use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;


#[derive(Serialize, Deserialize)]
struct Paragraph {
	name : String
}

#[derive(Serialize, Deserialize)]
struct Article {
	article : String,
	author : String,
	paragraph : Vec<Paragraph>
}

fn main () {

	let article : Article = Article {
		article : String::from("Mon tout nouvel article"),
		author : String::from("TERRADE Richard"),
		paragraph : vec![
			Paragraph {
				name : String::from("Premier par")
			},
			Paragraph {
				name : String::from("Deuxième pafdqsfdsqfdqsr")
			},
			Paragraph {
				name : String::from("Fin...")
			}
		]
	};

	let json = serde_json::to_string(&article).unwrap();
	println!("The json is : {}", json);

	creer_json_file(&json).unwrap();
}

fn creer_json_file(content : &String) -> std::io::Result<()> {
    let mut file = File::create("foo.json")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
