use std::collections::HashMap;

#[derive(Default)]
pub struct Config {
	pub case_insensitive: bool,
	pub ignore_field_count: usize,
	pub ignore_char_count: usize,
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
		let line = line.to_string();

		let line = if self.config.ignore_field_count > 0 {
			line
				.split_whitespace()
				.skip(self.config.ignore_field_count)
				.collect()
		} else {
			line
		};

		let line = if self.config.ignore_char_count > 0 {
			line.chars().skip(self.config.ignore_char_count).collect()
		} else {
			line
		};

		let line = if self.config.case_insensitive {
			line.to_lowercase()
		} else {
			line
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

	#[test]
	fn ignore_field() {
		let input = "\
A	test
B	test";

		let config = Config {
			ignore_field_count: 1,
			..Default::default()
		};
		let mut counter = Counter::new(config);
		for line in input.lines() {
			counter.count(line);
		}

		let mut expected = HashMap::new();
		expected.insert("test".to_string(), 2);

		assert_eq!(expected, counter.counts);
	}

	#[test]
	fn ignore_char() {
		let input = "\
A_test
B_test";

		let config = Config {
			ignore_char_count: 2,
			..Default::default()
		};
		let mut counter = Counter::new(config);
		for line in input.lines() {
			counter.count(line);
		}

		let mut expected = HashMap::new();
		expected.insert("test".to_string(), 2);

		assert_eq!(expected, counter.counts);
	}

	#[test]
	fn ignore_field_and_char() {
		let input = "\
A	A_test
B	B_test";

		let config = Config {
			ignore_field_count: 1,
			ignore_char_count: 2,
			..Default::default()
		};
		let mut counter = Counter::new(config);
		for line in input.lines() {
			counter.count(line);
		}

		let mut expected = HashMap::new();
		expected.insert("test".to_string(), 2);

		assert_eq!(expected, counter.counts);
	}
}
