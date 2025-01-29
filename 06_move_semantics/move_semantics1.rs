fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {
    let vec0 = vec![1, 2, 3];
    println!("Original vec0: {:?}", vec0);

    let vec1 = fill_vec(vec0.clone());
    println!("Vec1 after fill_vec: {:?}", vec1);

    // Experimenting with modifying the original vec0 after cloning it
    println!("Vec0 after cloning and modification: {:?}", vec0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
