use super::Value;

pub trait VIndex {
	fn index_into<'uwu>(&self, v: &'uwu Value) -> Option<&'uwu Value>;
	fn index_into_mut<'uwu>(&self, v: &'uwu mut Value) -> Option<&'uwu mut Value>;
}

impl VIndex for usize {
	fn index_into<'uwu>(&self, v: &'uwu Value) -> Option<&'uwu Value> {
		if let Value::Array(item) = v {
			item.value.get(*self)
		} else {
			None
		}
	}
	fn index_into_mut<'uwu>(&self, v: &'uwu mut Value) -> Option<&'uwu mut Value> {
		if let Value::Array(item) = v {
			item.value.get_mut(*self)
		} else {
			None
		}
	}
}

impl VIndex for str {
	fn index_into<'uwu>(&self, v: &'uwu Value) -> Option<&'uwu Value> {
		if let Value::Object(item) = v {
			item.value.get(&self.into())
		} else {
			None
		}
	}
	fn index_into_mut<'uwu>(&self, v: &'uwu mut Value) -> Option<&'uwu mut Value> {
		if let Value::Object(item) = v {
			item.value.get_mut(&self.into())
		} else {
			None
		}
	}
}

impl VIndex for String {
	fn index_into<'uwu>(&self, v: &'uwu Value) -> Option<&'uwu Value> {
		self.as_str().index_into(v)
	}
	fn index_into_mut<'uwu>(&self, v: &'uwu mut Value) -> Option<&'uwu mut Value> {
		self.as_str().index_into_mut(v)
	}
}

impl<'a, T> VIndex for &'a T
where
	T: ?Sized + VIndex,
{
	fn index_into<'uwu>(&self, v: &'uwu Value) -> Option<&'uwu Value> {
		(**self).index_into(v)
	}
	fn index_into_mut<'uwu>(&self, v: &'uwu mut Value) -> Option<&'uwu mut Value> {
		(**self).index_into_mut(v)
	}
}
