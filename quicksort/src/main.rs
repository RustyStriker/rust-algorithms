fn main(){
    // This is the main function :)
    let mut vec = vec![5,10,555,13,3,97,151];
    
    println!("before: {:?}",vec);
    quicksort::quicksort_my_vector(&mut vec);

    println!("After:  {:?}\n\nIs Sorted: {}",vec, is_vec_sorted(&vec));
}

/// Apperantly the Vec.is_sorted() function is not stable yet, so i made my own function
/// (probably not as fast tho worst case is O(n - 1) when the function is sorted)
/// * Please correct me if im wrong
fn is_vec_sorted<T : std::cmp::PartialOrd>(vec : &Vec<T>) -> bool{
    let len = vec.len() - 1;
    for i in 0..len{
        if vec[i] > vec[i + 1]{
            return false;
        }
    }
    true
}