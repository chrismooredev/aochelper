use std::fmt;
use std::io;

/// A tree node containing one unit of data, with an optional number of child nodes
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct TreeNode<T> {
	pub data: T,
	pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
	/// Create a terminal tree node with `data` and no children
	pub fn new(data: T) -> TreeNode<T> {
		TreeNode {
			data,
			children: Vec::new(),
		}
	}
	/// Create a non-terminal tree node with `data` and prefilled children
	pub fn new_with_children(data: T, children: Vec<TreeNode<T>>) -> TreeNode<T> {
		TreeNode { data, children }
	}

	pub fn has_children(&self) -> bool {
		self.children.len() > 0
	}
	/// Returns the direct number of children this node has. Returns 0 if terminal.
	pub fn len_children(&self) -> usize {
		self.children.len()
	}
	pub fn contains_recursive(&self, reference: &T) -> bool
	where
		T: PartialEq + Eq,
	{
		if &self.data == reference {
			return true;
		}
		self.children
			.iter()
			.any(|node| node.contains_recursive(reference))
	}
}

use std::borrow::Cow;
impl<T: fmt::Display + Clone> ptree::TreeItem for TreeNode<T> {
	type Child = Self;

	fn write_self<W: io::Write>(&self, f: &mut W, _style: &ptree::Style) -> io::Result<()> {
		// Only write the display of this data. If the consumer wants to use ptree for debug, they can implement it themselves
		write!(f, "{}", self.data)
	}
	fn children(&self) -> Cow<[Self::Child]> {
		Cow::from(&self.children)
	}
}
