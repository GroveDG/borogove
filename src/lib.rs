pub trait Boro: Sized {
	type Item;

	fn next(&mut self, i: Self::Item) -> bool;
	fn start(&mut self, s: Self) -> bool;
	fn until(&mut self, f: impl FnMut(Self::Item) -> bool) -> Option<Self>;
	fn whilst(&mut self, f: impl FnMut(Self::Item) -> bool) -> Self;
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

pub trait BoroStr: Boro<Item = char> {
	fn alphas(&mut self) -> Self {
		self.whilst(char::is_alphabetic)
	}
	fn nums(&mut self) -> Self {
		self.whilst(char::is_numeric)
	}
	fn word(&mut self) -> Self {
		self.whilst(|c| !char::is_whitespace(c))
	}
	fn white(&mut self) -> Self {
		self.whilst(char::is_whitespace)
	}
	fn line(&mut self) -> Option<Self> {
		self.until(|c| c == '\n')
	}
}

impl<T: Boro<Item = char>> BoroStr for T {}
