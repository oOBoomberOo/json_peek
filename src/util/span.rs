use std::fmt;
use std::ops::RangeInclusive;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Span {
	pub start: usize,
	pub end: usize,
}

impl Span {
	pub const fn new(start: usize, end: usize) -> Span {
		Span { start, end }
	}

	pub const fn range(&self) -> RangeInclusive<usize> {
		self.start..=self.end
	}

	pub const fn start_point(&self) -> Span {
		Span {
			start: self.start,
			end: self.start,
		}
	}

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

	pub const fn trim(&self, offset: usize) -> Span {
		Span::new(self.start + offset, self.end - offset)
	}

	#[allow(clippy::wrong_self_convention)]
	pub fn from_span(left: Span, right: Span) -> Span {
		let start = left.start.min(right.start);
		let end = left.end.max(right.end);
		Span::new(start, end)
	}

	#[cfg(test)]
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
