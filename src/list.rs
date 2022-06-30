use std::fmt;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List::Cons(head, tail) => {
                write!(f, "{} ", head)?;
                return tail.fmt(f);
            }
            List::Nil => write!(f, ""),
        }
    }
}

pub fn cons(head: i32, tail: List) -> List {
    return List::Cons(head, Box::new(tail));
}

pub fn nil() -> List {
    return List::Nil;
}

pub fn run() {
    let n1 = cons(1, nil());
    let n2 = cons(2, n1);
    let n3 = cons(3, n2);

    println!("{}", n3);
}
