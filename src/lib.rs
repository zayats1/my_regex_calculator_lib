pub use self::my_regex_calculator_lib::calculate;
pub mod my_regex_calculator_lib {
    use lazy_static::lazy_static;
    use regex:: Regex;

    pub fn calculate(input: String) -> String {
        let mut data = parse_string(input);
        let operators = ["*", "/", "-", "+"];

        // deep expressions  in parentheses
        for s in &mut data {
            if s.chars().count() > 5{
                let mut expression = s.clone();
                expression.remove(0);
                expression.pop();
                *s = calculate(expression);
            }
        }
        // if it has expressions  in parentheses ()
        // Todo optimization
        if data.contains(&"(".to_string()) && data.contains(&")".to_string()) {
            data = parse_string(calculate(data.join(" ")));
        }

        for op in operators {
            while data.contains(&op.to_string()) {
                let i = data.iter().position(|s| s == op).unwrap();
                let previous = i - 1;
                let next = i + 1;

                if data[i] == str::to_string(op) {
                    let num1 = data[previous].parse::<f64>();
                    let num2 = data[next].parse::<f64>();

                    if num1.is_err() || num2.is_err() {
                        break;
                    }

                    let num1 = num1.unwrap();
                    let num2 = num2.unwrap();

                    let calc_result = match math_operation(op, num1, num2){
                        Ok(_) => math_operation(op, num1, num2).ok(),
                        Err(e) => {
                            return e
                        }
                    };

                    data.remove(next);
                    data[i] = calc_result.unwrap().to_string();
                    data.remove(previous);
                }
            }
        }

        data.join(" ")
    }


    fn parse_string(input: String) -> Vec<String> {
        /*
            find double
            r"[+-]?([0-9]+\.?[0-9]*|\.[0-9]+)
            or math operator
            |(\+|\-|/|\*)|\(|\)
            The pattern that matches substrings in parentheses having no other ( and ) characters in between
            \([^()]*\)
            The pattern don't work
            \(.*\)
     */
        // to ensure that regular expressions are compiled exactly once.
        lazy_static! {
        static ref FIND_NUMBERS:Regex = Regex::new(r"\([^()]*\)|[+-]?([0-9]+\.?[0-9]*|\.[0-9]+)|(\+|\-|/|\*)|\(|\)").unwrap();
    }
        FIND_NUMBERS
            .find_iter(&input)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect()
    }


    fn math_operation(op: &str, num1: f64, num2: f64) -> Result<f64, String> {
        match op {
            "*" => {
               Ok(num1 * num2)
            }
            "/" => {
                if num2 != 0.0 {
                  Ok(num1 / num2)
                } else {
                    Err("Division by zero".to_string())
                }
            }
            "+" => {
                Ok(num1 + num2)
            }
            "-" => {
                Ok(num1 - num2)
            }

            _ => {
                Err("Unimplemented operator".to_string())
            }
        }
    }
}
