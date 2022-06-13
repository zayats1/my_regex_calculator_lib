#[cfg(test)]
mod tests {
    use my_regex_calculator_lib::my_regex_calculator_lib::calculate;

    #[test]
    fn it_works() {
        let my_data: String = "5 * (5 * (1 + 2)) - 5 * 5 + (5 + 5) / 5  - (-1.5)".parse().unwrap();
        let result = calculate(my_data);
        assert_eq!(result, "53.5");
    }
}
