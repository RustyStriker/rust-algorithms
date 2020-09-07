use hamming;

fn main(){
    // Create an 11 bit message to ham(?)
    let message = [
                        true,
            true, true , true,
            true, false, false,
            true , false , true , false
        ];

    println!("Original: {:?}",message);
    // Turn it into a 16 bit hamming message
    let mut hamming = hamming::to_hamming_code(&message);

    println!("Hamming: {:?}",hamming);

    hamming[4] ^= true; // Flip a bit to create an error

    println!("Error located at: {}",hamming::find_error_in_hamming(&hamming));

    println!("Deciphered: {:?}", hamming::decipher_hamming_code(&hamming));

    hamming[7] ^= true; // Flip for a second error

    println!("Error search: {}", hamming::find_error_in_hamming(&hamming));
}