#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let vec = vec!(0,1,5,8,10,15,222);

        assert_eq!(Some(5),binary_search(&vec, 5));
    }
    #[test]
    fn get_none(){
        let vec = vec![0,5,10,22,152,153,155,160];

        assert_eq!(None,binary_search(&vec, 15));
    }
}

pub fn binary_search<T : std::cmp::PartialOrd + std::cmp::Eq>(sorted_vec : &Vec<T>, value : T) -> Option<T>{
    binary_search_loop(&sorted_vec, value, 0, sorted_vec.len())
}

fn binary_search_loop<T : std::cmp::PartialOrd + std::cmp::Eq>(sorted_vec : &Vec<T>, value : T, start : usize, end : usize) -> Option<T>{
    if end - start <= 1{
        return None;
    }
    let current_index = ((end - start) / 2) + start;
    if value == sorted_vec[current_index]{
        return Some(value);
    }
    else if value < sorted_vec[current_index]{
        return binary_search_loop(&sorted_vec, value, start, current_index);
    }
    else{
        return binary_search_loop(&sorted_vec, value, current_index, end);
    }
}
