#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_in_bit_10() {
        let message = [
            false, false, true , false,
            true , false, true , true,
            true , false, true , false,
            true , true , true , false
        ];
        assert_eq!(find_error_in_hamming(&message), 10);
    }

    #[test]
    fn no_error(){
        let message = [
            false, false, true , false,
            true , false, true , true,
            true , false, false , false,
            true , true , true , false
        ];
        assert_eq!(find_error_in_hamming(&message), 0);
    }

    #[test]
    fn multiple_errors(){
        let message = [
            false, false, true , false,
            true , true, true , true,
            true , false, false , false,
            true , false , true , false
        ];
        assert_eq!(find_error_in_hamming(&message), 17);
    }
    #[test]
    fn fix_error_in_10() {
        let message = [
            false, false, true , false,
            true , false, true , true,
            true , false, true , false,
            true , true , true , false
        ];
        let corrected = [
                                 false,
                   false, true , true,
                   false, false, false,
            true , true , true , false
        ];

        assert_eq!(decipher_hamming_code(&message), Some(corrected));
    }
    #[test]
    fn to_and_from_direct(){
        let message = [
                        false,
            false, true , true,
            false, false, false,
            true , true , true , false
        ];

        assert_eq!(Some(message), decipher_hamming_code(&to_hamming_code(&message)));
    }
}

pub fn to_hamming_code(bits : &[bool ; 11]) -> [bool ; 16]{
    let mut result = [false ; 16];

    {
        // Inserting the bits to the final result in correct places
        let mut pos = 0;
        for i in 0..16{
            if ![0,1,2,4,8].contains(&i){
                result[i] = bits[pos];
                pos += 1;
            }
        }
    }
    result[1] = result[3] ^ result[5] ^ result[7] ^ result[9] ^ result[11] ^ result[13] ^ result[15];
    result[2] = result[3] ^ result[6] ^ result[7] ^ result[10] ^ result[11] ^ result[14] ^ result[15];
    result[4] = result[5] ^ result[6] ^ result[7] ^ result[12] ^ result[13] ^ result[14] ^ result[15];
    result[8] = result[9] ^ result[10] ^ result[11] ^ result[12] ^ result[13] ^ result[14] ^ result[15];
    let mut has_multiple_or_non = false;
    for b in result.iter(){
        if *b{
            has_multiple_or_non ^= true;
        }
    }
    result[0] = has_multiple_or_non;

    // Remove comment to see debug log
    //eprintln!("Got:    {:?}\nResult: {:?}",bits,result);

    result    
}

/// Returns the deciphered message if 1 error or less is presented
/// Returned None if 2 or more errors are presented
pub fn decipher_hamming_code(bits : &[bool ; 16]) -> Option<[bool ; 11]>{
    // Check if we have an error
    let error = find_error_in_hamming(bits);
    if error == 17{
        return None;
    }

    // Extract the data itself
    let mut result = [false; 11];
    let mut pos = 0;
    for i in 0..16{
        if ![0,1,2,4,8].contains(&i){
            if error as usize == i{
                result[pos] = !bits[i];
            }
            else{
                result[pos] = bits[i];
            }
            pos += 1;
        }
    }

    // Remove comment to see debug log
    //eprintln!("Got: {:?}\nExtracted: {:?}\nWith error at {}",bits,result,error);

    Some(result)
}


pub fn find_error_in_hamming(bits : &[bool ; 16]) -> i8{
    // Check for a general error
    let mut has_multiple_or_non = false;
    for b in bits{
        if *b{
            has_multiple_or_non ^= true;
        }
    }
    
    let b1 : bool = bits[1] ^ bits[3] ^ bits[5] ^ bits[7] ^ bits[9] ^ bits[11] ^ bits[13] ^ bits[15];
    let b2 : bool = bits[2] ^ bits[3] ^ bits[6] ^ bits[7] ^ bits[10] ^ bits[11] ^ bits[14] ^ bits[15];
    let b3 : bool = bits[4] ^ bits[5] ^ bits[6] ^ bits[7] ^ bits[12] ^ bits[13] ^ bits[14] ^ bits[15];
    let b4 : bool = bits[8] ^ bits[9] ^ bits[10] ^ bits[11] ^ bits[12] ^ bits[13] ^ bits[14] ^ bits[15];

    let position : i8 = b1 as i8 + b2 as i8 * 2 + b3 as i8 * 4 + b4 as i8 * 8;

    if !has_multiple_or_non && position != 0{
        return 17;
    }
    position
}