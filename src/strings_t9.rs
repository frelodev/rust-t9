use triple_accel::*;

pub struct Levenshtein {}

pub struct Hamming {}

pub trait T9Completer {
	fn distance(&self, a: &String, b: &String) -> u32;

	fn complete(&self, input: String, dictionary: Vec<String>) -> Vec<(String, u32)> {
		let mut min = std::u32::MAX;
		let mut out = Vec::new();
		for word in dictionary {
			let dist = self.distance(&input, &word);
			if dist < min {
				min = dist;
				out.clear();
				out.push((word, min));
			} else if dist == min {
				out.push((word, min));
			}
		}
		return out;
	}
}

impl T9Completer for Levenshtein {
	/// Levenshtein distance between two strings
	/// ```
	/// let completer = Levenshtein { };
	/// assert!(completer.complete("asd".into(), "asd".into()) == 1);
	/// ```
	fn distance(&self, a: &String, b: &String) -> u32 {
		levenshtein_exp(a.as_bytes(), b.as_bytes())
	}
}

impl T9Completer for Hamming {
	fn distance(&self, a: &String, b: &String) -> u32 {
		if a.len() != b.len() {
			return std::u32::MAX;
		}
		hamming(a.as_bytes(), b.as_bytes())
	}
}
