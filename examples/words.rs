pub fn main() {
	let text =
"'Twas brillig, and the slithy toves
Did gyre and gimble in the wabe;
All mimsy were the borogoves,
And the mome raths outgrabe.
";

	{
		use borogove::{Boro, BoroStr};

		let mut boro = text;

		while let Some(mut line) = boro.line() {
			println!("{}", line.word());
			boro.next('\n');
		}
	}
}
