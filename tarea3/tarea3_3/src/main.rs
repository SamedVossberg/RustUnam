pub trait IncreasingExt: Iterator
where
    Self::Item: PartialOrd + Clone,
{
    fn increasing(self) -> Increasing<Self>
    where
        Self: Sized,
    {
        Increasing::new(self)
    }
}

// trait for iterators -> this was a bit weird
impl<I> IncreasingExt for I
where
    I: Iterator,
    I::Item: PartialOrd + Clone,
{}

// Increasing struct for iter
pub struct Increasing<I>
where
    I: Iterator,
    I::Item: PartialOrd + Clone,
{
    iter: I,
    last: Option<I::Item>,
}

// methods for Increasing
impl<I> Increasing<I>
where
    I: Iterator,
    I::Item: PartialOrd + Clone,
{
    fn new(iter: I) -> Self {
        Increasing { iter, last: None }
    }
}

// Iterator trait for Increasing
impl<I> Iterator for Increasing<I>
where
    I: Iterator,
    I::Item: PartialOrd + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.iter.next() {
            if let Some(ref last) = self.last {
                if current > last.clone() {
                    self.last = Some(current.clone());
                    return Some(current);
                } else {
                    continue;
                }
            } else {
                self.last = Some(current.clone());
                return Some(current);
            }
        }
        None
    }
}

fn main() {
    let strings = vec![
        String::from("foo"),
        String::from("bar"),
        String::from("zoo"),
        String::from("art"),
    ];

    for s in strings.into_iter().increasing() {
        println!("{s}");
    }

    let numbers = vec![1, 2, 4, 2, 1, 5, 0];
    for n in numbers.into_iter().increasing() {
        println!("{n}");
    }
}
