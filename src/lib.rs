
use lazy_static::lazy_static;
use regex::Regex;

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


fn calculate(input: String) -> String {
    let mut data = parse_string(input.clone());
    let operators = ["*", "/", "-", "+"];

    // deep expressions  in parentheses
    for s in &mut data {
        if s.chars().count() > 5 {
            let mut expression = s.clone();
            expression.remove(0);
            expression.pop();
            *s = calculate(expression);
        }
    }
    // if it has expressions  in parentheses ()
    // Todo optimization
    if data.contains(&"(".to_string()) && data.contains(&")".to_string()) {
        data = parse_string(calculate(data.join(" ").to_owned()));
    }

    for op in operators {
        while data.contains(&op.to_string()) {
            let i = data.iter().position(|s| s == op).unwrap();
            let previous = i - 1;
            let next = i + 1;

            if data[i] == op.to_string() {
                let num1 = data[previous].parse::<f64>();
                let num2 = data[next].parse::<f64>();

                if num1.is_err() || num2.is_err() {
                    break;
                }

                let num1 = num1.unwrap();
                let num2 = num2.unwrap();

                let calc_result = math_operation(op, num1, num2);

                data.remove(next);
                data[i] = calc_result.to_string();
                data.remove(previous);
            }
        }
    }

    return data.join(" ").to_owned();
}

fn math_operation(op: &str, num1: f64, num2: f64) -> f64 {
    match op {
        "*" => {
            num1 * num2
        }
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                panic!("division by zero");
            }
        }
        "+" => {
            num1 + num2
        }
        "-" => {
            num1 - num2
        }

        _ => {
            0.0
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn it_works() {
        let my_data: String = "5 * (5 * (1 + 2)) - 5 * 5 + (5 + 5) / 5  - (-1.5)".parse().unwrap();
        let result = calculate(my_data);
        assert_eq!(result, "53.5");
    }
}
