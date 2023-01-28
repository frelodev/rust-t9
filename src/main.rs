mod IOUtil;
mod strings_t9;

use IOUtil::{DictLoader, FileDictLoader};
use clap::Parser;
use strings_t9::{Levenshtein, T9Completer, Hamming};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Input string to complete
	input: String,

	/// Path of dictionary file, one word per line
	#[arg(short, long, default_value = "parole.txt")]
	dictionary: String,

	/// Algorithm to use for distance
	#[arg(short, long, default_value = "levenshtein")]
	algorithm: String,
}

fn main() -> Result<(), std::io::Error> {
	let args = Args::parse();

	let algo : Option<Box<dyn T9Completer>> = match args.algorithm.to_lowercase().trim() {
		"levenshtein" => { Some(Box::new(Levenshtein {})) },
		"hamming"     => { Some(Box::new(Hamming {})) },
		_             => { None },
	};

	let completer = algo.unwrap();

	let qualcosa = FileDictLoader {};
	let dictionary = qualcosa.load_from(args.dictionary);
	let results = completer.complete(args.input, dictionary);
	for (word, dist) in results{
		println!("> {} ({})", word, dist);
	}

	Ok(())
}
