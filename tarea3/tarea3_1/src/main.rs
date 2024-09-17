fn increasing_u32(input: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    

    // Struct for our custom iterator
    struct Increasing<I> {
        iter: I,
        prev: Option<u32>,
    }

    impl<I: Iterator<Item = u32>> Iterator for Increasing<I> {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(current) = self.iter.next() {
                if let Some(prev) = self.prev {
                    // If the current value is greater than the previous one, return it
                    if prev < current {
                        self.prev = Some(current);
                        return Some(current);
                    }
                } else {
                    // If there is no previous value, return the current one
                    self.prev = Some(current);
                    return Some(current);
                }

                // If the current value is not greater than the previous one, update the previous value
                self.prev = Some(current);
            }

            // if no more values -> none 
            None
        }
    }

    // Return of custom iterator
    Increasing {
        iter: input,
        prev: None,
    }
}

fn main() {
    let v = vec![1, 2, 4, 2, 1, 5, 0];
    for x in increasing_u32(v.into_iter()) {
        print!("{x} ");
    }
}
