use std::{
	fmt,
	ops::{Add, AddAssign, RangeInclusive, Sub, SubAssign},
};

/// General position information of string
///
/// ```
/// # use json_peek::util::Span;
/// let a = Span::new(1, 5);
/// let b = Span::from(1..=5);
///
/// assert_eq!(a, b);
/// ```
///
/// ```
/// # use json_peek::util::Span;
/// let span = Span::new(2, 4);
/// span.trim(1); // Equal to: Span::new(3, 3);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Span {
	pub start: usize,
	pub end: usize,
}

impl Span {
	pub const fn new(start: usize, end: usize) -> Span {
		Span { start, end }
	}

	/// Get the range of this Span
	pub const fn range(&self) -> RangeInclusive<usize> {
		self.start..=self.end
	}

	/// Create a point out of the start position of this Span
	pub const fn start_point(&self) -> Span {
		Span {
			start: self.start,
			end: self.start,
		}
	}

	/// Create a point out of the end position of this Span
	pub const fn end_point(&self) -> Span {
		Span {
			start: self.end,
			end: self.end,
		}
	}

	/// Zero-width `Span` is considered to be a point
	pub const fn is_point(&self) -> bool {
		self.start == self.end
	}

	/// Shrink the size of Span by some `offset` from both size
	///
	/// **Does not check for overflow**
	/// ```
	/// # use json_peek::util::Span;
	/// let span = Span::new(2, 4);
	///
	/// assert_eq!(span.trim(1), Span::new(3, 3));
	/// assert_eq!(span.trim(2), Span::new(4, 2));
	/// ```
	///
	/// You can cause integer underflow/overflow with this method if not careful.
	/// ```should_panic
	/// # use json_peek::util::Span;
	/// let span = Span::new(0, 4);
	/// span.trim(5); // <- will panic!!
	/// ```
	pub const fn trim(&self, offset: usize) -> Span {
		Span::new(self.start + offset, self.end - offset)
	}

	/// Create Span from two other Spans
	/// This function will construct the biggest possible Span
	/// 
	/// ```
	/// # use json_peek::util::Span;
	/// let a = Span::new(1, 5);
	/// let b = Span::new(4, 10);
	/// 
	/// assert_eq!(Span::from_span(a, b), Span::new(1, 10));
	/// ```
	pub fn from_span(left: Span, right: Span) -> Span {
		let start = left.start.min(right.start);
		let end = left.end.max(right.end);
		Span::new(start, end)
	}

	/// Span value for testing purpose
	pub const fn test() -> Span {
		Span::new(0, 0)
	}
}

impl Add<usize> for Span {
	type Output = Span;

	fn add(self, rhs: usize) -> Span {
		Span {
			start: self.start,
			end: self.end + rhs,
		}
	}
}

impl Sub<usize> for Span {
	type Output = Span;

	fn sub(self, rhs: usize) -> Span {
		Span {
			start: self.start,
			end: self.end - rhs,
		}
	}
}

impl AddAssign<usize> for Span {
	fn add_assign(&mut self, rhs: usize) {
		self.end += rhs;
	}
}

impl SubAssign<usize> for Span {
	fn sub_assign(&mut self, rhs: usize) {
		self.end -= rhs;
	}
}

impl Add<Span> for Span {
	type Output = Span;

	fn add(self, rhs: Span) -> Span {
		Span {
			start: self.start + rhs.start,
			end: self.end + rhs.end,
		}
	}
}

impl Sub<Span> for Span {
	type Output = Span;

	fn sub(self, rhs: Span) -> Span {
		Span {
			start: self.start - rhs.start,
			end: self.end - rhs.end,
		}
	}
}

impl AddAssign<Span> for Span {
	fn add_assign(&mut self, rhs: Span) {
		self.start += rhs.start;
		self.end += rhs.end;
	}
}

impl fmt::Debug for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}

impl fmt::Display for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}

impl From<RangeInclusive<usize>> for Span {
	fn from(range: RangeInclusive<usize>) -> Span {
		Span::new(*range.start(), *range.end())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_range() {
		assert_eq!(Span::new(3, 9).range(), 3..=9);
	}

	#[test]
	fn add_usize() {
		assert_eq!(Span::new(3, 9) + 1, Span { start: 3, end: 10 });
	}

	#[test]
	fn sub_usize() {
		assert_eq!(Span::new(0, 10) - 10, Span { start: 0, end: 0 });
	}

	#[test]
	fn add_span() {
		assert_eq!(
			Span::new(0, 9) + Span::new(5, 8),
			Span { start: 5, end: 17 }
		);
	}

	#[test]
	fn sub_span() {
		assert_eq!(
			Span::new(5, 10) - Span::new(3, 4),
			Span { start: 2, end: 6 }
		);
	}
}
