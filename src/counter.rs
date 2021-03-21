use std::collections::HashMap;

pub struct Counter {
	pub counts: HashMap<String, u32>,
}

impl Counter {
	pub fn new() -> Counter {
		Counter {
			counts: HashMap::new(),
		}
	}

	pub fn count(&mut self, line: &str) {
		*self.counts.entry(line.to_string()).or_insert(0) += 1;
	}

	pub fn count_case_insensitive(&mut self, line: &str) {
		let line = line.to_lowercase();
		self.count(&line);
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

		let mut counter = Counter::new();
		for line in input.lines() {
			counter.count(line);
		}

		let mut result: HashMap<String, u32> = HashMap::new();
		result.insert("test".to_string(), 2);
		result.insert("Test".to_string(), 1);

		assert_eq!(result, counter.counts);
	}

	#[test]
	fn case_insensitive() {
		let input = "\
test
Test
test";

		let mut counter = Counter::new();
		for line in input.lines() {
			counter.count_case_insensitive(line);
		}

		let mut result: HashMap<String, u32> = HashMap::new();
		result.insert("test".to_string(), 3);

		assert_eq!(result, counter.counts);
	}
}
