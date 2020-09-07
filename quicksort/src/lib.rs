#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![0,6,32,15,2];
        quicksort_my_vector(&mut vec);

        assert_eq!(vec![0,2,6,15,32],vec);
    }
}

/// Quickly sorts an array of items(with the PrtialOrd trait)
/// Simply pass in a mutable array and the function will sort it
pub fn quicksort_my_vector<T : std::cmp::PartialOrd>(mut vec : &mut Vec<T>){
    // This is basically a helper function so the user doesnt have to pass more than the vector itself
    let len = vec.len();

    quicksort(&mut vec, 0, len);
}

/// The actualt quicksort function, using recursion and a pivot function
fn quicksort<T : std::cmp::PartialOrd>(mut vec : &mut Vec<T>, start : usize, end : usize){
    // Organize the left side - using recursion
    // Organize the right side - using recursion    
    
    if start < end{
        let pivot = pivot(&mut vec, start, end);

        quicksort(&mut vec, start, pivot);
        quicksort(&mut vec, pivot + 1, end);
    }
}

fn pivot<T : std::cmp::PartialOrd>(vec : &mut Vec<T>, start: usize, end : usize) -> usize{
    //let pivot = vec[end - 1];
    let mut i = start;

    for j in start..end - 1{
        if vec[j] < vec[end - 1]{
            // Do swap with i
            vec.swap(j,i);
            // Increase i by 1
            i += 1;
        }
    }
    // Repalce pivot point with i
    vec.swap(i,end - 1);

    i
}