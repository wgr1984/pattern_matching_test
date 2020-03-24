use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

enum Test<T> where T: Display  {
    TestNested(T, Box<Test<T>>),
    Empty,
}

impl<T> Test<T> where T: Display {
    fn iter(&self) -> impl Iterator<Item=&T> {
        Iter { next: self, _phantom: PhantomData }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        MutIter { next: self, _phantom: PhantomData }
    }
}

pub struct Iter<'a, T> where T: Display {
    next: *const Test<T>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> where T: Display {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        unsafe {
            match &*self.next {
                Test::Empty => None,
                Test::TestNested(v, next) => {
                    self.next = next.as_ref();
                    Some(&v)
                }
            }
        }
    }
}

pub struct MutIter<'a, T> where T: Display {
    next: *mut Test<T>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for MutIter<'a, T> where T: Display {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        unsafe {
            match *self.next {
                Test::Empty => None,
                Test::TestNested(ref mut v, ref mut next) => {
                    self.next = next.as_mut();
                    Some(v)
                }
            }
        }
    }
}


impl<T: Display> Display for Test<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        fn enum_to_string<T>(test: &Test<T>, f: &mut Formatter<'_>) -> Result where T: Display {
            write!(f, "[")?;
            for (i, t) in test.iter().enumerate() {
                let t_str = t.to_string();
                if t_str.is_empty() {
                    continue
                }
                if i == 0 {
                    write!(f, "{}", t_str)?;
                    continue
                }
                write!(f, ", {}", t_str)?;
            }
            write!(f, "]")
        }

        enum_to_string(&self, f)
    }
}

fn main() {
    let mut test = Test::TestNested(
        "Hello".to_owned(),
        Box::new(Test::TestNested("World".to_owned(), Box::new(Test::Empty))),
    );

    println!("{}", test);

    for t in test.iter_mut() {
        t.push_str("2");
    }

    println!("{}", test);
}
