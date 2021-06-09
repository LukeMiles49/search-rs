use replace_with::replace_with_or_abort;
use std::rc::Rc;

#[derive(Debug)]
pub enum List<T> {
	Nil,
	Cons(Rc<(T, List<T>)>),
}

impl<T> Clone for List<T> {
	fn clone(&self) -> List<T> {
		match *self {
			List::Nil => List::Nil,
			List::Cons(ref rc) => List::Cons(rc.clone()),
		}
	}
}

impl<T> List<T> {
	pub fn cons(x: T, xs: List<T>) -> List<T> {
		List::Cons(Rc::new((x, xs)))
	}
	
	pub fn head(&self) -> Option<&T> {
		match self {
			List::Nil => None,
			List::Cons(ref rc) => Some(&rc.0),
		}
	}
	
	pub fn tail(&self) -> Option<List<T>> {
		match self {
			List::Nil => None,
			List::Cons(ref rc) => Some(rc.1.clone()),
		}
	}
	
	pub fn iter<'a>(&'a self) -> ListIter<'a, T> {
		ListIter(self)
	}
}

pub struct ListIter<'a, T>(&'a List<T>);

impl<'a, T> Iterator for ListIter<'a, T> {
	type Item = &'a T;
	
	fn next(&mut self) -> Option<&'a T> {
		let mut result = None;
		replace_with_or_abort(&mut self.0, |s| match s {
			List::Nil => { result = None; &List::Nil },
			List::Cons(rc) => { result = Some(&rc.0); &rc.1 },
		});
		result
	}
}
