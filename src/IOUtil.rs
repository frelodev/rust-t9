use std::fs::File;
use std::io::{prelude::*, BufReader};

pub trait DictLoader {
	fn load_from(&self, path: String) -> Vec<String>;
}

pub struct FileDictLoader {}

impl DictLoader for FileDictLoader {
	fn load_from(&self, path: String) -> Vec<String> {
		let file = File::open(path).unwrap();
		let reader = BufReader::new(file);
		let mut dict_strings: Vec<String> = Vec::new();
		for line in reader.lines() {
			match line {
				Ok(l) => dict_strings.push(l),
				Err(e) => {
					eprintln!("Oops error {:?}", e)
				}
			}
		}

		return dict_strings;
	}
}
