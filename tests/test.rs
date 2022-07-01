#[cfg(test)]
mod tests {
    use my_regex_calculator_lib::calculate;

    #[test]
    fn it_works() {
        let my_data: String = "5 * (5 * (1 + 2)) - 5 * 5 + (5 + 5) / 5  - (-1.5)".parse().unwrap();
        let result = calculate(my_data);
        assert_eq!(result, "53.5");
    }

    #[test]
    fn calculation_order() {
        let my_data: String = "2 + 2 - 2 * 2 / 2 ".parse().unwrap();
        let result = calculate(my_data);
        assert_eq!(result, "2");
    }

    #[test]
    fn division_by_zero() {
        let my_data: String = "1 / 0 ".parse().unwrap();
        eprintln!("{}", calculate(my_data))
    }
}
