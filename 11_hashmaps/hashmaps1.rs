// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    
    // Two bananas are already given
    basket.insert(String::from("banana"), 2);
    
    // Adding more fruits
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 1);
    
    basket
}

fn main() {
    let basket = fruit_basket();
    println!("Fruit basket contains: {:?}", basket);
    
    // Running tests in main
    assert!(basket.len() >= 3, "There should be at least three types of fruits.");
    assert!(basket.values().sum::<u32>() >= 5, "Total fruit count should be at least five.");
    println!("Checks passed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}