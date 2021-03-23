use std::collections::HashMap;

#[derive(Default)]
pub struct Config {
	pub case_insensitive: bool,
	pub ignore_field_count: u32,
	pub ignore_char_count: u32,
}

pub struct Counter {
	pub counts: HashMap<String, u32>,
	config: Config,
}

impl Counter {
	pub fn new(config: Config) -> Counter {
		Counter {
			counts: HashMap::new(),
			config,
		}
	}

	pub fn count(&mut self, line: &str) {
		let line = if self.config.case_insensitive {
			line.to_lowercase()
		} else {
			line.to_string()
		};

		*self.counts.entry(line).or_insert(0) += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let input = "\
test
Test
test";

		let config = Default::default();
		let mut counter = Counter::new(config);
		for line in input.lines() {
			counter.count(line);
		}

		let mut expected = HashMap::new();
		expected.insert("test".to_string(), 2);
		expected.insert("Test".to_string(), 1);

		assert_eq!(expected, counter.counts);
	}

	#[test]
	fn case_insensitive() {
		let input = "\
test
Test
test";

		let config = Config {
			case_insensitive: true,
			..Default::default()
		};
		let mut counter = Counter::new(config);
		for line in input.lines() {
			counter.count(line);
		}

		let mut expected = HashMap::new();
		expected.insert("test".to_string(), 3);

		assert_eq!(expected, counter.counts);
	}
}
