#[cfg(test)]
mod tests {
    use rug::Integer;
    #[test]
    fn it_works() {
        let input = 2u128.pow(63) + 7;
        let mut value = Integer::from(input.clone());
        let mut f = || {
            value = Integer::from(value.square_ref());
        };
        f();
        assert_eq!(Integer::from(input.pow(2)), value);
    }
}
