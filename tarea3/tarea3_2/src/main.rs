fn increasing_generic<T>(input: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    T: PartialOrd + Clone,
{
    struct Increasing<I, T>
    where
        I: Iterator<Item = T>,
        T: PartialOrd + Clone,
    {
        iter: I,
        max_so_far: Option<T>,
    }

    impl<I, T> Iterator for Increasing<I, T>
    where
        I: Iterator<Item = T>,
        T: PartialOrd + Clone,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            for current in self.iter.by_ref() {
                if let Some(ref max) = self.max_so_far {
                    if current > *max {
                        self.max_so_far = Some(current.clone());
                        return Some(current);
                    }
                } else {
                    self.max_so_far = Some(current.clone());
                    return Some(current);
                }
            }
            None
        }
    }

    Increasing {
        iter: input,
        max_so_far: None,
    }
}


fn main() {
    let v = vec![1, 2, 4, 2, 1, 5, 0];
    for x in increasing_generic(v.into_iter()) {
        print!("{x} ");
    }
    println!();

    let words = vec!["aber", "holari", "mein", "kaktus", "holari", "holari"];
    for word in increasing_generic(words.into_iter()) {
        print!("{word} ");
    }
    println!();
}