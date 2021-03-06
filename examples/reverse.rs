extern crate quickcheck;

use quickcheck::quickcheck;

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

fn main() {
    fn equality_after_applying_twice(xs: Vec<int>) -> bool {
        xs == reverse(reverse(xs.as_slice()).as_slice())
    }
    quickcheck(equality_after_applying_twice as fn(Vec<int>) -> bool);
}
