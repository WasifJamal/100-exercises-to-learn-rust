pub fn example() -> usize {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // I have changed the str to &str as &str has the size. It stores a pointer and length of the str
    std::mem::size_of::<&str>()
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn should_return_size() {
        assert_eq!(size_of::<usize>() * 2, example());
    }
}
