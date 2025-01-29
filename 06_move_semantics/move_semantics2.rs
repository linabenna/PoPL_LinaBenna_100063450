fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {
    // Experimenting with vec0 and vec1
    let vec0 = vec![1, 2, 3];
    println!("Original vec0: {:?}", vec0);

    let vec1 = fill_vec(vec0.clone()); // Clone vec0 to keep it intact
    println!("Vec1 after fill_vec: {:?}", vec1);

    // vec0 remains unchanged
    println!("Vec0 after cloning and modification: {:?}", vec0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // Clone vec0 to retain access to the original vector
        let vec1 = fill_vec(vec0.clone());

        // Now vec0 is still accessible
        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}


