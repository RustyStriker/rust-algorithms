#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_general_test() {
        let vec = vec![2,-3,5,10,-8,9,-9];
        assert_eq!(16, kadane(&vec));
    }
    #[test]
    fn range_test(){
        let vec = vec![2,-3,5,10,-8,9,-9];
        assert_eq!((16,2,6),kadane_with_range(&vec));
    }
}

pub fn kadane(vec : &Vec<i32>) -> i32{
    let min =  0;
    let mut best_sum = min;
    let mut current = min;

    println!("Starting iteration");
    for i in vec.iter(){
        // Uncomment for better view(maybe add some more println!)
        // println!("Iteration {} current {} best {}",i,current,best_sum);
        current += i;

        if best_sum < current{
            best_sum = current;
        }
        if current < min{
            current = min;
        }
    }
    
    best_sum
}

/// Returns (MAX VALUE, START INDEX, END INDEX - excluded)
/// For example kadane_with_range([2,-3,5,10,-8,9,-9]) -> (16,2,6)
pub fn kadane_with_range(vec : &Vec<i32>) -> (i32,usize,usize){
    // Simply a copy paste from the original one with a few lines added

    let min =  0;
    let mut best_sum = min;
    let mut current = min;
    // added 2 variables
    let mut start : usize = 0;
    let mut end : usize = 0;


    println!("Starting iteration");
    for t in 0..vec.len(){ // Changed to have iter number instead of just values
        // Uncomment for better view(maybe add some more println!)
        // println!("Iteration {} current {} best {}",i,current,best_sum);
        let i = vec[t];
        

        if current <= min{
            start = t; // reset the start
            current = i;
        }
        else{
            current += i;
        }

        if best_sum < current{
            best_sum = current;
            end = t + 1; // update the end
        }
    }
    
    (best_sum,start,end)
}