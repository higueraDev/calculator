use regex::Regex;

fn operations(re: Regex, expression: String, operation_type: &str) -> String {
    let mut exp = expression;

    loop {
        let caps = re.captures(exp.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        // operation str
        let cap_expression = caps.get(0).unwrap().as_str();
        // first value
        let left_value: f32 = caps.get(1).unwrap().as_str().parse().unwrap();
        // second value
        let right_value: f32 = caps.get(2).unwrap().as_str().parse().unwrap();
        // acc
        let mut operation = 0.00;

        if operation_type == "/" {
            operation = left_value / right_value;
        } else if operation_type == "*" {
            operation = left_value * right_value;
        } else if operation_type == "+" {
            operation = left_value + right_value;
        } else if operation_type == "-" {
            operation = left_value - right_value;
        }

        let mut positive_expression = "+".to_owned();
        positive_expression.push_str(cap_expression);
        let negative_expression = exp.replace(positive_expression.as_str(), cap_expression);
        
        println!("{}: {}", cap_expression, operation);

        if operation.is_sign_negative() {
            exp = negative_expression.replace(cap_expression, operation.to_string().as_str());
        } else {
            exp = exp.replace(cap_expression, operation.to_string().as_str());
        }
    }
    return exp;
}

fn main() {
    // Regex
    let re_add = Regex::new(r"(\-?\d+\.?\d*)\s*\+\s*(\-?\d+\.?\d*)").unwrap();
    let re_sub = Regex::new(r"(\-?\d+\.?\d*)\s*\-\s*(\-?\d+\.?\d*)").unwrap();
    let re_mult = Regex::new(r"(\-?\d+\.?\d*)\s*\*\s*(\-?\d+\.?\d*)").unwrap();
    let re_div = Regex::new(r"(\-?\d+\.?\d*)\s*/\s*(\-?\d+\.?\d*)").unwrap();

    // User inputs
    println!("Please type the expression without parenthesis ()");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    //Operations
    println!("--- Details ---");
    expression = operations(re_mult, expression, "*");
    expression = operations(re_div, expression, "/");
    expression = operations(re_add, expression, "+");
    expression = operations(re_sub, expression, "-");

    // Result
    println!("--- Done ---");
    println!("The result is: {}", expression);
}
