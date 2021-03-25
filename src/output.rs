use std::collections::HashMap;
use std::io::Write;

pub struct Config {
	pub debug: bool,
}

pub struct Output {
	sink: Box<dyn Write>,
	config: Config,
	last_height: usize,
}

impl Output {
	pub fn new(sink: Box<dyn Write>, config: Config) -> Output {
		Output {
			sink,
			config,
			last_height: 0,
		}
	}

	pub fn print(&mut self, counts: &HashMap<String, u32>) {
		let mut pairs: Vec<(&String, &u32)> = counts.iter().collect();
		pairs.sort_by(|a, b| a.1.cmp(b.1).reverse());

		if !self.config.debug {
			// For each previous line in the output, move the cursor up and clear the line.
			for _ in 0..self.last_height {
				self
					.sink
					.write_all("\u{001B}[F\u{001B}[K".as_ref())
					.unwrap();
			}
		}
		for (line, count) in pairs {
			self
				.sink
				.write_all(format!("{}\t{}\n", count, line).as_ref())
				.unwrap();
		}
		self.sink.flush().unwrap();

		self.last_height = counts.len();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn print_prints() {
		// TODO!
		//  add item to map
		//  print
		//  validate output
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
	fn debug_does_not_use_ansi_control_codes() {
		// TODO!
		//  add item to map
		//  print
		//  add new item to map
		//  print
		//  validate no ANSI codes are used
	}
}
