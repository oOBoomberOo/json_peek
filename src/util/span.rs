use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
	pub start: usize,
	pub end: usize
}

impl Span {
	pub const fn new(start: usize, end: usize) -> Span {
		Span { start, end }
	}

	pub const fn range(&self) -> RangeInclusive<usize> {
		self.start..=self.end
	}
}

impl Add<usize> for Span {
	type Output = Span;

	fn add(self, rhs: usize) -> Span {
		Span {
			start: self.start,
			end: self.end + rhs
		}
	}
}

impl Sub<usize> for Span {
	type Output = Span;

	fn sub(self, rhs: usize) -> Span {
		Span {
			start: self.start,
			end: self.end - rhs
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
			end: self.end + rhs.end
		}
	}
}

impl AddAssign<Span> for Span {
	fn add_assign(&mut self, rhs: Span) {
		self.start += rhs.start;
		self.end += rhs.end;
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
		assert_eq!(Span::new(0, 9) + Span::new(5, 8), Span { start: 5, end: 17 });
	}
}