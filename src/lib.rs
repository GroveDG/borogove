#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// General parsing trait.
pub trait Boro: Sized {
	/// Element type of the slice.
	type Item;

	/// Take one `Item` from the slice.
	fn one(&mut self) -> Option<Self::Item>;
	/// Take one `Item`, returns wether it exists.
	fn next(&mut self, i: Self::Item) -> bool;
	/// Take one `Item` if `f` is true.
	fn then(&mut self, f: impl FnMut(Self::Item) -> bool) -> bool;
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
		let content = self.until(|i| i == close)?;
		self.next(close);
		Some(content)
	}
}

/// Reverse of [`Boro`].
pub trait Orob: Boro {
	/// Reverse of [`Boro::one`].
	fn back(&mut self) -> Option<Self::Item>;
	/// Reverse of [`Boro::next`].
	fn last(&mut self, i: Self::Item) -> bool;
	/// Reverse of [`Boro::then`].
	fn thence(&mut self, f: impl FnMut(Self::Item) -> bool) -> bool;
	/// Reverse of [`Boro::start`].
	fn end(&mut self, s: Self) -> bool;
	/// Reverse of [`Boro::until`].
	fn since(&mut self, f: impl FnMut(Self::Item) -> bool) -> Option<Self>;
	/// Reverse of [`Boro::whilst`].
	fn hence(&mut self, f: impl FnMut(Self::Item) -> bool) -> Self;
}

impl Boro for &str {
	type Item = char;

	fn one(&mut self) -> Option<char> {
		let one = self.chars().next()?;
		*self = &self[one.len_utf8()..];
		Some(one)
	}
	fn next(&mut self, i: char) -> bool {
		let mut s = *self;
		if s.one().is_some_and(|c| c == i) {
			*self = s;
			return true;
		}
		false
	}
	fn then(&mut self, f: impl FnMut(Self::Item) -> bool) -> bool {
		let mut s = *self;
		if s.one().is_some_and(f) {
			*self = s;
			return true;
		}
		false
	}
	fn start(&mut self, s: Self) -> bool {
		if self.starts_with(s) {
			*self = &self[s.len()..];
			return true;
		}
		false
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

impl Orob for &str {
	fn back(&mut self) -> Option<char> {
		let one = self.chars().rev().next()?;
		*self = &self[one.len_utf8()..];
		Some(one)
	}
	fn last(&mut self, i: char) -> bool {
		let mut s = *self;
		if s.back().is_some_and(|c| c == i) {
			*self = s;
			return true;
		}
		false
	}
	fn thence(&mut self, f: impl FnMut(Self::Item) -> bool) -> bool {
		let mut s = *self;
		if s.back().is_some_and(f) {
			*self = s;
			return true;
		}
		false
	}
	fn end(&mut self, s: Self) -> bool {
		if self.ends_with(s) {
			*self = &self[..self.len() - s.len()];
			return true;
		}
		false
	}
	fn since(&mut self, mut f: impl FnMut(char) -> bool) -> Option<Self> {
		let mut chars = self.char_indices().rev();
		let start = loop {
			// If we run out of chars...
			let Some((i, c)) = chars.next() else {
				// ... return None.
				*self = &self[..0];
				return None;
			};
			// If the char matches...
			if f(c) {
				// ... return the end of this char.
				break i + c.len_utf8();
			}
		};
		let tmp = *self;
		*self = &tmp[..start];
		Some(&tmp[start..])
	}
	fn hence(&mut self, mut f: impl FnMut(char) -> bool) -> Self {
		let mut chars = self.char_indices().rev();
		let start = loop {
			// If all chars are matching and we run out...
			let Some((i, c)) = chars.next() else {
				// ... return the full string.
				break 0;
			};
			// If the char doesn't match...
			if !f(c) {
				// ... return the end of this char.
				break i + c.len_utf8();
			}
		};
		let tmp = *self;
		*self = &tmp[..start];
		&tmp[start..]
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
