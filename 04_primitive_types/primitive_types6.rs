fn main() {
    let numbers = (1, 2, 3);
    let second = numbers.1; // Accessing the second element
    println!("The second number is {}", second);
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Accessing the second element using tuple indexing
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
