# Borogove

Borogove is a simple personal parsing library. It is built around the following function signature:

```
fn name(&mut &str, ...) -> &str
```

This means we take a mutable reference to a slice of input. When the input is parsed, the input slice is trimmed, gradually consuming the input, and the parsed segment is returned. This pattern helps make the process of parsing more straightforward, if a little verbose.

## Example
```
pub fn main() {
	let text =
"'Twas brillig, and the slithy toves
Did gyre and gimble in the wabe;
All mimsy were the borogoves,
And the mome raths outgrabe.";

	{
		use borogove::{Boro, BoroText};

		let mut boro = text;

		while let Some(mut line) = boro.line() {
			println!("{}", line.word());
			boro.next('\n');
		}
	}
}
```
```
'Twas
Did
All
And
```