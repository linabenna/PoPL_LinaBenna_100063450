fn main() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4]; // Extracting the slice
    println!("{:?}", nice_slice); // Verifying output
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // getting values from 1 to 3 inclusive
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
