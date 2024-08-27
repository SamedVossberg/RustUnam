fn main() {
    let array = &mut[3,9,1,2,3,4,1];
    let sorted_int_vec = mysort_int(array);
    println!("Sorted int array {:?}", sorted_int_vec);
    println!("Sorted generic array {:?}", mysort(array));
}

fn mysort_int(slice: &mut [i32]) -> Vec<i32> {
    let mut vec = slice.to_vec();
    vec.sort();
    return vec;
}

fn mysort<T: Ord+Clone>(slice: &[T]) -> Vec<T> {
    let mut vec = slice.to_vec();
    vec.sort();
    return vec;
}
