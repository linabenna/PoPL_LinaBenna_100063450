fn main() {
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42); // Use y first before creating z
    let z = &mut x;
    z.push(13);
    println!("ans = {:?}", x);
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42); // y has to use x first before z can refer to x
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}