fn parse(expression: &str) -> i64 {
    let seperated_exps: Vec<String> = Vec::new();

    let res: Vec<(String, i64)> = extract_between_parentheses(expression);

    print!("Between arithmetic operators: {:?}\n", res);

    res[0].1
}

fn calculate(left: i64, right: i64, operator: char) -> i64 {
    let mut result: i64 = 0;

    match operator {
        '+' => result = left + right,
        '-' => result = left - right,
        '*' => result = left * right,
        '/' => result = left / right,
        _ => panic!("Invalid operator: `{}`", operator),
    }

    result
}

fn calculate_expressions(expressions: Vec<String>) -> i64 {
    0
}

fn calculate_expression(expression: &str) -> i64 {
    let mut result: i64 = 0;
    let mut current_expression: String = expression.to_string();

    if current_expression.len() == 0 {
        return 0;
    }

    while let Some(index) =
        current_expression.find(|c| c == '+' || c == '-' || c == '*' || c == '/')
    {
        let ch = current_expression.chars().nth(index).unwrap();
        let operand = ch.clone();

        let left_exp = &current_expression[..current_expression.find(ch).unwrap()];
        let right_exp = &current_expression[current_expression.find(ch).unwrap() + 1..];
        let right_first_num_str: &str = right_exp
            .split(&['+', '-', '*', '/'])
            .collect::<Vec<&str>>()
            .first()
            .unwrap();
        let end_index_for_expr = index + right_first_num_str.len();

        let left_num = left_exp.parse::<i64>().unwrap();
        let right_num = right_first_num_str.parse::<i64>().unwrap();
        result = calculate(left_num, right_num, operand);

        current_expression = format!(
            "{}{}",
            result,
            current_expression[end_index_for_expr + 1..].to_string()
        );
    }
    result
}

// fn extract_between_parentheses(input: &str) -> Vec<(String, i64)> {
//     let mut input = input.to_string();
//     print!("DEBUG:extract_between_parentheses:Input: `{}`\n", input);
//     let mut final_calculation: i64 = 0;
//     let mut result: Vec<(String, i64)> = Vec::new();

//     while let Some(start) = input.find('(') {
//         while let Some(end) = input[start..].rfind(')') {
//             let expr = input[start + 1..start + end].to_string();
//             print!("DEBUG:extract_between_parentheses:expr: `{}`\n", expr);
//             let extracted = extract_between_parentheses(expr.as_str());
//             print!(
//                 "DEBUG:extract_between_parentheses:extracted: `{:?}`\n",
//                 extracted
//             );

//             println!("Will update input...");
//             let expr_calc = extracted.last().unwrap().1;
//             print!(
//                 "DEBUG:extract_between_parentheses:expr_calc: {}\n",
//                 expr_calc
//             );

//             let exp_to_replace = format!("({})", expr);
//             print!(
//                 "DEBUG:extract_between_parentheses:exp_to_replace: `{}`\n",
//                 exp_to_replace
//             );

//             print!("DEBUG:extract_between_parentheses:input: `{}`\n", input);
//             input = input.replace(&exp_to_replace, expr_calc.to_string().as_str());
//             print!(
//                 "DEBUG:extract_between_parentheses:input after replacing expr: `{}`\n",
//                 input
//             );

//             final_calculation += expr_calc;
//             print!("DEBUG:extract_between_parentheses:final_calculation after adding expr_calc: `{}`\n", final_calculation);

//             let item_to_add = (expr.to_string(), expr_calc);
//             print!(
//                 "DEBUG:extract_between_parentheses:item_to_add: `{:?}`\n",
//                 item_to_add
//             );
//             result.push(item_to_add);
//             print!(
//                 "DEBUG:extract_between_parentheses:result after pushing item_to_add: `{:?}`\n",
//                 result
//             );
//             return extracted;
//         }
//     }

//     let calculation = calculate_expression(input.as_str());
//     result.push((input.to_string(), calculation));
//     print!("DEBUG:extract_between_parentheses:result: `{:?}`\n", result);
//     result
// }

// ==============

fn extract_between_parentheses(input: &str) -> Vec<(String, i64)> {
    let mut input = input.to_string();
    print!("DEBUG:extract_between_parentheses:Input: `{}`\n", input);
    let mut final_calculation: i64 = 0;
    let mut result: Vec<(String, i64)> = Vec::new();

    while let Some(start) = input.find('(') {
        while let Some(end) = input[start..].rfind(')') {
            let expr = input[start + 1..start + end].to_string();
            print!("DEBUG:extract_between_parentheses:expr: `{}`\n", expr);

            let extracted = extract_between_parentheses(expr.as_str());
            print!(
                "DEBUG:extract_between_parentheses:extracted: `{:?}`\n",
                extracted
            );

            // let expr_calc = extracted.iter().map(|(_, val)| val).sum::<i64>();
            // print!(
            //     "DEBUG:extract_between_parentheses:expr_calc: {}\n",
            //     expr_calc
            // );

            println!("Will update input...");
            let expr_calc = extracted.last().unwrap().1;
            print!(
                "DEBUG:extract_between_parentheses:expr_calc: {}\n",
                expr_calc
            );

            let exp_to_replace = format!("({})", expr);
            print!(
                "DEBUG:extract_between_parentheses:exp_to_replace: `{}`\n",
                exp_to_replace
            );

            input = input.replace(&exp_to_replace, expr_calc.to_string().as_str());
            print!(
                "DEBUG:extract_between_parentheses:input after replacing expr: `{}`\n",
                input
            );

            let calc_new_input = calculate_expression(&input);
            print!(
                "DEBUG:extract_between_parentheses:calc_new_input: `{}`\n",
                calc_new_input
            );

            final_calculation += calc_new_input;
            print!(
                "DEBUG:extract_between_parentheses:final_calculation after adding calc_new_input: `{}`\n",
                final_calculation
            );

            let item_to_add = (expr.to_string(), final_calculation);
            print!(
                "DEBUG:extract_between_parentheses:item_to_add: `{:?}`\n",
                item_to_add
            );
            result.push(item_to_add);
            print!(
                "DEBUG:extract_between_parentheses:result after pushing item_to_add: `{:?}`\n",
                result
            );
        }
    }

    let calculation = calculate_expression(input.as_str());
    result.push((input.to_string(), calculation));
    print!("DEBUG:extract_between_parentheses:result: `{:?}`\n", result);
    result
}

// ==============

fn determine_arithmetic_operators(expression: &str) -> String {
    let mut result: String = String::new();

    for ch in expression.chars() {
        match ch {
            'a' => result.push('+'),
            'b' => result.push('-'),
            'c' => result.push('*'),
            'd' => result.push('/'),
            'e' => result.push('('),
            'f' => result.push(')'),
            _ => result.push(ch),
        }
    }

    result
}

fn main() {
    let inputs = vec![
        "3a2c4",
        "32a2d2",
        "500a10b66c32",
        "3ae4c66fb32",
        "3c4d2aee2a4c41fc4f",
    ];

    let mut input_arith: Vec<String> = vec![];

    // Determine Arithmetic Operators
    for input in inputs {
        let result = determine_arithmetic_operators(input);
        input_arith.push(result.clone());
        println!("Input: \"{}\"\nResult: {}\n", input, result);
    }

    println!("===================");

    // Calculate Arithmetic Operators
    for input in input_arith {
        // println!("Input: \"{}\"", input);
        let result = parse(&input);
        println!("Final: {} = {}\n", input, result);
    }
}
