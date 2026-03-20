#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// General parsing trait.
pub trait Boro: Sized {
	/// Element type of the slice.
	type Item;

	/// Take one `Item`, returns wether it exists.
	fn next(&mut self, i: Self::Item) -> bool;
	/// Take a `Self` slice, returns wether it exists.
	fn start(&mut self, s: Self) -> bool;
	/// Take until `f` returns true, returns `None` if never true.
	fn until(&mut self, f: impl FnMut(Self::Item) -> bool) -> Option<Self>;
	/// Take while `f` returns true.
	fn whilst(&mut self, f: impl FnMut(Self::Item) -> bool) -> Self;
	/// Take all between `open` and `close`, returns `None` if either does not exist.
	fn block(&mut self, open: Self::Item, close: Self::Item) -> Option<Self>
	where
		Self::Item: PartialEq,
	{
		if !self.next(open) {
			return None;
		}
		self.until(|i| i == close)
	}
}

impl Boro for &str {
	type Item = char;

	fn next(&mut self, i: char) -> bool {
		let n = self.starts_with(i);
		if n {
			*self = &self[i.len_utf8()..];
		}
		n
	}
	fn start(&mut self, s: Self) -> bool {
		self.starts_with(s)
	}
	fn until(&mut self, mut f: impl FnMut(char) -> bool) -> Option<Self> {
		let mut chars = self.char_indices();
		let end = loop {
			// If we run out of chars...
			let Some((i, c)) = chars.next() else {
				// ... return None.
				*self = &self[self.len()..];
				return None;
			};
			// If the char matches...
			if f(c) {
				// ... return the start of this char.
				break i;
			}
		};
		let tmp = *self;
		*self = &tmp[end..];
		Some(&tmp[..end])
	}
	fn whilst(&mut self, mut f: impl FnMut(char) -> bool) -> Self {
		let mut chars = self.char_indices();
		let end = loop {
			// If all chars are matching and we run out...
			let Some((i, c)) = chars.next() else {
				// ... return the full string.
				break self.len();
			};
			// If the char doesn't match...
			if !f(c) {
				// ... return the start of this char.
				break i;
			}
		};
		let tmp = *self;
		*self = &tmp[end..];
		&tmp[..end]
	}
}

/// Text parsing trait.
pub trait BoroText: Boro<Item = char> {
	/// Take while [`char::is_alphabetic`] is true.
	fn alphas(&mut self) -> Self {
		self.whilst(char::is_alphabetic)
	}
	/// Take while [`char::is_numeric`] is true.
	fn nums(&mut self) -> Self {
		self.whilst(char::is_numeric)
	}
	/// Take while [`char::is_whitespace`] is false.
	fn word(&mut self) -> Self {
		self.whilst(|c| !char::is_whitespace(c))
	}
	/// Take while [`char::is_whitespace`] is true.
	fn white(&mut self) -> Self {
		self.whilst(char::is_whitespace)
	}
	/// Take until `'\n'`.
	fn line(&mut self) -> Option<Self> {
		self.until(|c| c == '\n')
	}
}

impl<T: Boro<Item = char>> BoroText for T {}
