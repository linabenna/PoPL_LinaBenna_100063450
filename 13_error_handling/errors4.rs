#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // This function must handle negative inputs and 0s 
        if value < 0 {
            Err(CreationError::Negative) // error for -ve values
        } else if value == 0{
            Err(CreationError::Zero) // error for 0
        } else {
            Ok(Self(value as u64)) // returns Ok
        }
    }
}

fn main() {
    // You can optionally experiment here.
    match PositiveNonzeroInteger::new(-5) {
        Ok(num) => println!("Created: {:?}", num),
        Err(err) => println!("Error: {:?}", err),
    }
    match PositiveNonzeroInteger::new(0) {
        Ok(num) => println!("Created: {:?}", num),
        Err(err) => println!("Error: {:?}", err),
    }
    match PositiveNonzeroInteger::new(5) {
        Ok(num) => println!("Created: {:?}", num),
        Err(err) => println!("Error: {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
