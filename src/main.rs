fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(3, 3); // Fix this line! Both sides should be equal.
    }
}