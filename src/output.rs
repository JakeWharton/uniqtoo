use std::collections::HashMap;
use std::error::Error;
use std::io::Write;

pub struct Config {
	pub reverse: bool,
	pub head: Option<usize>,
	pub debug: bool,
}

pub struct Output<'a> {
	sink: &'a mut dyn Write,
	config: Config,
	last_height: usize,
}

impl<'a> Output<'a> {
	pub fn new(sink: &mut dyn Write, config: Config) -> Output {
		Output {
			sink,
			config,
			last_height: 0,
		}
	}

	pub fn print(&mut self, counts: &HashMap<String, u32>) -> Result<(), Box<dyn Error>> {
		let mut pairs: Vec<(&String, &u32)> = counts.iter().collect();

		// Sort items with higher counts to the top.
		pairs.sort_by(|a, b| b.1.cmp(a.1));

		if self.config.reverse {
			pairs.reverse();
		}

		if let Some(head) = self.config.head {
			pairs.truncate(head);
		}

		if !self.config.debug {
			// For each previous line in the output, move the cursor up and clear the line.
			self
				.sink
				.write_all("\u{001B}[F\u{001B}[K".repeat(self.last_height).as_ref())?;
		}
		for (line, count) in &pairs {
			self
				.sink
				.write_all(format!("{}\t{}\n", count, line).as_ref())?;
		}
		self.sink.flush()?;

		self.last_height = pairs.len();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn print(config: Config, counts: &HashMap<String, u32>) -> Result<String, Box<dyn Error>> {
		let mut sink = Vec::new();
		let mut output = Output::new(&mut sink, config);
		output.print(counts)?;
		Ok(String::from_utf8(sink)?)
	}

	#[test]
	fn print_prints() -> Result<(), Box<dyn Error>> {
		let config = Config {
			reverse: false,
			head: None,
			debug: false,
		};

		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 2);
		counts.insert("B".to_string(), 1);
		counts.insert("C".to_string(), 3);

		let actual = print(config, &counts)?;

		let expected = "\
3	C
2	A
1	B
"
		.to_string();

		assert_eq!(expected, actual);
		Ok(())
	}

	#[test]
	fn print_reverse() -> Result<(), Box<dyn Error>> {
		let config = Config {
			reverse: true,
			head: None,
			debug: false,
		};

		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 2);
		counts.insert("B".to_string(), 1);
		counts.insert("C".to_string(), 3);

		let actual = print(config, &counts)?;

		let expected = "\
1	B
2	A
3	C
"
		.to_string();

		assert_eq!(expected, actual);
		Ok(())
	}

	#[test]
	fn print_clears_previous() -> Result<(), Box<dyn Error>> {
		let config = Config {
			reverse: false,
			head: None,
			debug: false,
		};
		let mut sink = Vec::new();
		let mut output = Output::new(&mut sink, config);

		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 1);
		counts.insert("B".to_string(), 2);

		output.print(&counts)?;

		counts.insert("C".to_string(), 3);
		counts.insert("D".to_string(), 4);

		output.print(&counts)?;

		let actual = String::from_utf8(sink)?;

		let expected = "\
2	B
1	A
\u{001B}[F\u{001B}[K\
\u{001B}[F\u{001B}[K\
4	D
3	C
2	B
1	A
"
		.to_string();

		assert_eq!(expected, actual);
		Ok(())
	}

	#[test]
	fn head_limits_output() -> Result<(), Box<dyn Error>> {
		let config = Config {
			reverse: false,
			head: Some(2),
			debug: false,
		};
		let mut sink = Vec::new();
		let mut output = Output::new(&mut sink, config);

		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 1);
		counts.insert("B".to_string(), 2);
		counts.insert("C".to_string(), 3);
		counts.insert("D".to_string(), 4);

		output.print(&counts)?;

		counts.insert("E".to_string(), 5);
		counts.insert("F".to_string(), 6);

		output.print(&counts)?;

		let actual = String::from_utf8(sink)?;

		let expected = "\
4	D
3	C
\u{001B}[F\u{001B}[K\
\u{001B}[F\u{001B}[K\
6	F
5	E
"
		.to_string();

		assert_eq!(expected, actual);
		Ok(())
	}

	#[test]
	fn head_limits_output_reversed() -> Result<(), Box<dyn Error>> {
		let config = Config {
			reverse: true,
			head: Some(2),
			debug: false,
		};
		let mut sink = Vec::new();
		let mut output = Output::new(&mut sink, config);

		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 3);
		counts.insert("B".to_string(), 4);
		counts.insert("C".to_string(), 5);
		counts.insert("D".to_string(), 6);

		output.print(&counts)?;

		counts.insert("E".to_string(), 1);
		counts.insert("F".to_string(), 2);

		output.print(&counts)?;

		let actual = String::from_utf8(sink)?;

		let expected = "\
3	A
4	B
\u{001B}[F\u{001B}[K\
\u{001B}[F\u{001B}[K\
1	E
2	F
"
		.to_string();

		assert_eq!(expected, actual);
		Ok(())
	}

	#[test]
	fn debug_does_not_use_ansi_control_codes() -> Result<(), Box<dyn Error>> {
		let config = Config {
			reverse: false,
			head: None,
			debug: true,
		};
		let mut sink = Vec::new();
		let mut output = Output::new(&mut sink, config);

		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 1);
		counts.insert("B".to_string(), 2);

		output.print(&counts)?;

		counts.insert("C".to_string(), 3);
		counts.insert("D".to_string(), 4);

		output.print(&counts)?;

		let actual = String::from_utf8(sink)?;

		let expected = "\
2	B
1	A
4	D
3	C
2	B
1	A
"
		.to_string();

		assert_eq!(expected, actual);
		Ok(())
	}
}
