use std::collections::HashMap;
use std::error::Error;
use std::io::Write;

pub struct Config {
	pub reverse: bool,
	pub head: Option<usize>,
	pub debug: bool,
}

pub struct Output<'a> {
	sink: Box<&'a mut dyn Write>,
	config: Config,
	last_height: usize,
}

impl<'a> Output<'a> {
	pub fn new(sink: Box<&mut dyn Write>, config: Config) -> Output {
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

	#[test]
	fn print_prints() {
		//  add item to map
		let mut counts = HashMap::new();
		counts.insert("A".to_string(), 2);
		counts.insert("B".to_string(), 1);
		counts.insert("C".to_string(), 3);

		//  print
		let config = Config {
			reverse: false,
			head: None,
			debug: false,
		};
		let mut sink = Vec::new();
		let mut output = Output::new(Box::new(&mut sink), config);

		output.print(&counts).unwrap();

		//  validate output is sorted
		let expected = "\
3	C
2	A
1	B
"
		.to_string();

		assert_eq!(expected, String::from_utf8(sink).unwrap());
	}

	#[test]
	fn print_reverse() {
		// TODO!
		//  config { reverse: true }
		//  add item to map
		//  print
		//  validate output is sorted in reverse
	}

	#[test]
	fn print_clears_previous() {
		// TODO!
		//  add item to map
		//  print
		//  add new item to map
		//  print
		//  validate ANSI codes
	}

	#[test]
	fn head_limits_output() {
		// TODO!
		//  config { head: 2 }
		//  add 4 item to map
		//  print
		//  validate top 2 printed
		//  add new item to map that sorts to top
		//  print
		//  validate new top 2 printed
	}

	#[test]
	fn head_limits_output_reversed() {
		// TODO!
		//  config { head: 2, reverse: true }
		//  add 4 item to map
		//  print
		//  validate bottom 2 printed
		//  add new item to map that sorts to top
		//  print
		//  validate new buttom 2 printed
	}

	#[test]
	fn debug_does_not_use_ansi_control_codes() {
		// TODO!
		//  add item to map
		//  print
		//  add new item to map
		//  print
		//  validate no ANSI codes are used
	}
}
