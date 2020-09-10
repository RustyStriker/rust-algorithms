#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        assert_eq!(gcd(333,636),3);
    }

    #[test]
    fn extended_test() {
        assert_eq!((3,-21,11),gcd_extended(333, 636));
    }
}

/// Basically using modulos( % ) to represent a as "a = bx + y"
/// We can dictate from here that a's greatest common divisor is also b and y's gcd 
pub fn gcd(a : i32, b : i32) -> i32{
    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

pub fn gcd_extended(a : i32, b : i32) -> (i32,i32,i32) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (g, x, y) = gcd_extended(b % a, a);

    (g , (y - (b/a) * x), x)
}