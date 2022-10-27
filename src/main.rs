/*
 Modular multiplicative index using Euclidean algorithm requires two computations
  i) For Computing Gcd which will help in determining the quotient required to perform Mod inverse. the GCD will be calculated using Euclidean algorithm
 ii) The function which we will use to compute mode inverse using the values obtained from the first function


 First we create euclid_gcd  function which finds the greatest common divisor of two integers a and b,
 i.e value_a and value_b as paramenters
 such that ( value_a· x) + ( value_b · y ) is the greatest common divisor of value_a and value_b · x
 using the extended Euclidean algorithm.
 
 prerequisites = value of a must be greater than value of b ie (a > b)

*/
fn euclid_gcd(a: isize, b: isize) -> (isize, isize, isize) {

    // As long as value a is not equal to 0, this condition will run, otherwise gcd will be equal to the next value, which is b
    if a != 0 {

        // When computing euclid gcd, we first find the mod of the two given values 
    
        let mod_val = b % a;

        // Since this process of finding GCD is repetitive , we will use the mod_val to compute
        // euclid_gcd recursively 
        // After the euclid function is run recursively, the gcd_val will be obtained when the base function is reached
        // i.e when b = 0;
        // value_a and value_b obtained below will be used to compute mode inverse in the next function
        let (gcd_val, value_a, value_b) = euclid_gcd( mod_val,a);

        /*
        The steps below is used to return gcd and values that will be used in next function
        */ 

        let quotient = b / a;

        let new_value_a = value_b - (quotient * value_a);

        let new_value_b = value_a;

        // After the function has run and it has finished execution in all loops then this is what will be returned 
        return (gcd_val, new_value_a, new_value_b);
    }

    return (b, 0, 1);
}

/**
mod_inverse calculates the Modular Multiplicative Inverse
of number such that a·x ≡ 1 (mod m)
In the event that there will be no integer passed, this function will return None.
Otherwise, the inverse will be returned wrapped up in Some
*/
fn mod_inverse(a: isize, m: isize) -> Option<isize> {
    let (gcd, value_a, _) = euclid_gcd(a, m);
// When finding the inverse the GCD is always 1
    if gcd != 1 {
        return None;
    }
 
    return Some((value_a % m + m) % m);
}

// we import the std::io create in order to read from standard-input
use std::io;

fn main() {
    println!("Enter two values in order for Modular multiplicative Index to be calculated :\n");
    println!("i.e. 11 3 \n");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let inputs: Vec<isize> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let a = inputs[0];
        let m = inputs[1];

        let result = mod_inverse(a, m);
        match result {
            Some(inverse) => {
                println!("{a} and {m} have a Modular Multiplicative Inverse of -> {inverse} \n");
            }
            None => println!("Modular_Inverse can't be computed for {a} mod {m} because they are not co-prime\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mod_inverse;

    #[test]
    fn it_works() {
        assert!(
            mod_inverse(22, 11).is_none(),
            "modular inverse works if numbers are co-prime, therefore prime numbers wont be computed"
        );
        assert_eq!(mod_inverse(11, 3).unwrap(), 2);
        assert_eq!(mod_inverse(13, 11).unwrap(), 6);
        
    }
}
