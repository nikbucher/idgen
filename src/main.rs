use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::Shell;
use rand::distr::Uniform;
use rand::Rng;

#[derive(Parser)]
#[command(name = "idgen", version, about = "Generate random identifiers")]
struct IdGen {
	/// Alphabet to use for ID generation
	#[arg(short = 'a', long = "alphabet", default_value = "lowercase")]
	alphabet: Alphabet,

	/// Number of characters per block
	#[arg(short = 'b', long = "block-size", default_value_t = 4)]
	block_size: usize,

	/// Delimiter between blocks
	#[arg(short = 'd', long = "delimiter", default_value = "-")]
	delimiter: String,

	/// Total number of characters in the generated ID
	#[arg(short = 's', long = "size", default_value_t = 20)]
	size: usize,

	/// Number of IDs to generate
	#[arg(short = 'n', long = "count", default_value_t = 1)]
	count: usize,

	/// Suppress config output
	#[arg(short = 'q', long = "quiet")]
	quiet: bool,

	/// Generate shell completion script and exit
	#[arg(long = "completions", value_name = "SHELL", value_enum)]
	completions: Option<Shell>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Alphabet {
	/// NanoID / URL-safe Base64: _-0-9a-zA-Z
	Nanoid,
	/// All alphanumeric: 0-9a-zA-Z
	All,
	/// Ambiguity-safe uppercase: excludes B, D, I, O, S, Z
	Uppercase,
	/// All uppercase: 0-9A-Z
	AllUppercase,
	/// Ambiguity-safe lowercase: excludes 1, l
	Lowercase,
	/// All lowercase: 0-9a-z
	AllLowercase,
	/// Ambiguity-safe mixed case: excludes B, D, I, K, O, S, Z, k, l
	UppercaseAndLowercase,
}

impl Alphabet {
	fn chars(&self) -> &str {
		match self {
			Alphabet::Nanoid => "_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
			Alphabet::All => "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
			Alphabet::Uppercase => "34679ACEFGHJKLMNPQRTUVWXY",
			Alphabet::AllUppercase => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
			Alphabet::Lowercase => "023456789abcdefghijkmnopqrstuvwxyz",
			Alphabet::AllLowercase => "0123456789abcdefghijklmnopqrstuvwxyz",
			Alphabet::UppercaseAndLowercase => "34679ACEFGHJLMNPQRTUVWXYabcdefghijmnopqrstuvwxyz",
		}
	}
}

fn generate_id(alphabet: &Alphabet, size: usize) -> String {
	let chars = alphabet.chars().as_bytes();
	let mut rng = rand::rng();
	let dist = Uniform::new(0, chars.len()).unwrap();
	(0..size).map(|_| chars[rng.sample(dist)] as char).collect()
}

fn format_id(id: &str, block_size: usize, delimiter: &str) -> String {
	if block_size == 0 {
		return id.to_string();
	}
	id.chars()
		.collect::<Vec<_>>()
		.chunks(block_size)
		.map(|chunk| chunk.iter().collect::<String>())
		.collect::<Vec<_>>()
		.join(delimiter)
}

fn main() {
	let args = IdGen::parse();

	if let Some(shell) = args.completions {
		let mut cmd = IdGen::command();
		let bin_name = cmd.get_name().to_string();
		clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
		return;
	}

	if !args.quiet {
		eprintln!("|   alphabet: {:?}", args.alphabet);
		eprintln!("| block-size: {}", args.block_size);
		eprintln!("|  delimiter: {}", args.delimiter);
		eprintln!("|       size: {}", args.size);
		eprintln!("|      count: {}", args.count);
	}

	for _ in 0..args.count {
		let id = generate_id(&args.alphabet, args.size);
		println!("{}", format_id(&id, args.block_size, &args.delimiter));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_id_correct_length() {
		for size in [0, 1, 10, 20, 100] {
			let id = generate_id(&Alphabet::Lowercase, size);
			assert_eq!(id.len(), size, "expected length {size}");
		}
	}

	#[test]
	fn generate_id_uses_only_alphabet_chars() {
		let variants = [
			Alphabet::Nanoid,
			Alphabet::All,
			Alphabet::Uppercase,
			Alphabet::AllUppercase,
			Alphabet::Lowercase,
			Alphabet::AllLowercase,
			Alphabet::UppercaseAndLowercase,
		];
		for alphabet in &variants {
			let allowed = alphabet.chars();
			let id = generate_id(alphabet, 200);
			for ch in id.chars() {
				assert!(allowed.contains(ch), "char '{ch}' not in alphabet {:?} ({})", alphabet, allowed);
			}
		}
	}

	#[test]
	fn format_id_splits_into_blocks() {
		assert_eq!(format_id("abcdefgh", 4, "-"), "abcd-efgh");
		assert_eq!(format_id("abcdefgh", 3, "."), "abc.def.gh");
		assert_eq!(format_id("abcdefgh", 8, "-"), "abcdefgh");
	}

	#[test]
	fn format_id_block_size_larger_than_id() {
		assert_eq!(format_id("abc", 10, "-"), "abc");
	}

	#[test]
	fn format_id_empty_separator() {
		assert_eq!(format_id("abcdef", 3, ""), "abcdef");
	}

	#[test]
	fn format_id_empty_input() {
		assert_eq!(format_id("", 4, "-"), "");
	}

	#[test]
	fn format_id_block_size_zero() {
		assert_eq!(format_id("abcdef", 0, "-"), "abcdef");
	}

	#[test]
	fn all_alphabets_are_ascii_and_nonempty() {
		let variants = [
			Alphabet::Nanoid,
			Alphabet::All,
			Alphabet::Uppercase,
			Alphabet::AllUppercase,
			Alphabet::Lowercase,
			Alphabet::AllLowercase,
			Alphabet::UppercaseAndLowercase,
		];
		for alphabet in &variants {
			let chars = alphabet.chars();
			assert!(!chars.is_empty(), "{:?} is empty", alphabet);
			assert!(chars.is_ascii(), "{:?} contains non-ASCII", alphabet);
		}
	}

	#[test]
	fn completions_generate_for_all_shells() {
		for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell, Shell::Elvish] {
			let mut cmd = IdGen::command();
			let mut buf: Vec<u8> = Vec::new();
			clap_complete::generate(shell, &mut cmd, "idgen", &mut buf);
			assert!(!buf.is_empty(), "completion output empty for {shell:?}");
		}
	}
}
