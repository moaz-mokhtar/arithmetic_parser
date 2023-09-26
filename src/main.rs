fn parse(expression: &str) -> i64 {
    let seperated_exps: Vec<String> = Vec::new();

    let res: Vec<(String, i64)> = extract_between_parentheses(expression);

    print!("Between arithmetic operators: {:?}\n", res);

    res[0].1
}

fn calculate(left: i64, right: i64, operator: char) -> i64 {
    let mut result: i64 = 0;

    print!("DEBUG:calculate:left: `{}`\n", left);
    print!("DEBUG:calculate:right: `{}`\n", right);
    print!("DEBUG:calculate:operator: `{}`\n", operator);

    match operator {
        '+' => result = left + right,
        '-' => result = left - right,
        '*' => result = left * right,
        '/' => result = left / right,
        _ => panic!("Invalid operator: `{}`", operator),
    }

    result
}

/// Calculate an arithmetic expression
/// Example 1;
///     expression: 3+2*4
///     result: 20
/// Example 2:
///     expression: 32+2/2
///     result: 17
/// Returns arithmatic expression from left to right
// fn calculate_expression(expression: &str) -> i64 {
//     print!("DEBUG:calculate_expression:Input: `{}`\n", expression);

//     let mut result: i64 = 0;
//     let mut expression: String = expression.to_string();

//     if expression.len() == 0 {
//         return 0;
//     }

//     for (index, ch) in expression.chars().enumerate() {
//         print!("DEBUG:calculate_expression:ch: `{}` , index: `{}`\n", ch, index);
//         if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
//             let operand = ch.clone();
//             let left_exp = &expression[..expression.find(ch).unwrap()];
//             let right_exp = &expression[expression.find(ch).unwrap() + 1..];
//             let right_first_num_str: &str = right_exp
//                 .split(&['+', '-', '*', '/'])
//                 .collect::<Vec<&str>>()
//                 .first()
//                 .unwrap();
// 			let end_index_for_expr = index+right_first_num_str.len();

//             print!("DEBUG:calculate_expression:left_exp: `{}`\n", left_exp);
//             print!("DEBUG:calculate_expression:operand: `{}`\n", operand);
//             print!("DEBUG:calculate_expression:right_exp: `{}`\n", right_exp);
//             print!(
//                 "DEBUG:calculate_expression:right_first_num_str: `{}`\n",
//                 right_first_num_str
//             );
// 			print!("DEBUG:calculate_expression:end_index_for_expr: `{}`\n", end_index_for_expr);

//             let left_num = left_exp.parse::<i64>().unwrap();
//             let right_num = right_first_num_str.parse::<i64>().unwrap();
//             result = calculate(left_num, right_num, operand);
//             print!("DEBUG:calculate_expression:result: `{}`\n", result);

// 			expression = expression[end_index_for_expr..].to_string();
// 			print!("DEBUG:calculate_expression:expression: `{}`\n", expression);
//         }
//     }

//     // ============
//     // // If the expression is ready for operations, just do the operations
//     // let splitted_based_on_operands: Vec<&str> = expression
//     //     .split(&['+', '-', '*', '/'])
//     //     .collect::<Vec<&str>>();
//     // print!(
//     //     "DEBUG:calculate_expression:splitted_based_on_operands: `{:?}`\n",
//     //     splitted_based_on_operands
//     // );

//     // if splitted_based_on_operands.len() == 2 {

//     // } else if expression.len() == 1 {
//     //             return expression.parse::<i64>().unwrap();
//     // } else {
//     //     for ch in expression.chars() {
//     //         print!("DEBUG:calculate_expression:ch: `{}`\n", ch);
//     //         if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
//     //             let operand = ch.clone();
//     //             let left_exp = &expression[..expression.find(ch).unwrap()];
//     //             let right_exp = &expression[expression.find(ch).unwrap() + 1..];
//     //             print!("DEBUG:calculate_expression:left_exp: `{}`\n", left_exp);
//     //             print!("DEBUG:calculate_expression:operand: `{}`\n", operand);
//     //             print!("DEBUG:calculate_expression:right_exp: `{}`\n", right_exp);

//     //         }
//     //     }
//     // }

//     result
// }

fn calculate_expression(expression: &str) -> i64 {
    print!("DEBUG:calculate_expression:Input: `{}`\n", expression);

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

        print!("DEBUG:calculate_expression:left_exp: `{}`\n", left_exp);
        print!("DEBUG:calculate_expression:operand: `{}`\n", operand);
        print!("DEBUG:calculate_expression:right_exp: `{}`\n", right_exp);
        print!(
            "DEBUG:calculate_expression:right_first_num_str: `{}`\n",
            right_first_num_str
        );
        print!(
            "DEBUG:calculate_expression:end_index_for_expr: `{}`\n",
            end_index_for_expr
        );

        let left_num = left_exp.parse::<i64>().unwrap();
        let right_num = right_first_num_str.parse::<i64>().unwrap();
        result = calculate(left_num, right_num, operand);
        print!("DEBUG:calculate_expression:result: `{}`\n", result);

        current_expression = format!(
            "{}{}",
            result,
            current_expression[end_index_for_expr + 1..].to_string()
        );
        print!(
            "DEBUG:calculate_expression:current_expression: `{}`\n",
            current_expression
        );
    }
    print!("DEBUG:calculate_expression:result: `{}`\n", result);
    result
}

fn extract_between_parentheses(input: &str) -> Vec<(String, i64)> {
	let mut input = input.to_string();
    print!("DEBUG:extract_between_parentheses:Input: `{}`\n", input);
    let mut final_calculation: i64 = 0;
	let mut result: Vec<(String, i64)> = Vec::new();

    while let Some(start) = input.find('(') {
        while let Some(end) = input[start..].rfind(')') {
            let expr = &input[start + 1..start + end];
            print!("DEBUG:extract_between_parentheses:expr: `{}`\n", expr);
            let extracted = extract_between_parentheses(expr);
            print!(
                "DEBUG:extract_between_parentheses:extracted: `{:?}`\n",
                extracted
            );
			
			let expr_calc = extracted.last().unwrap().1;
			print!("DEBUG:extract_between_parentheses:expr_calc: `{}`\n", expr_calc);
			final_calculation += expr_calc;
            print!("DEBUG:extract_between_parentheses:final_calculation after adding expr_calc: `{}`\n", final_calculation);
			
			let item_to_add = (expr.to_string(), expr_calc);
            print!(
                "DEBUG:extract_between_parentheses:item_to_add: `{:?}`\n",
                item_to_add
            );
            result.push(item_to_add);
            print!(
                "DEBUG:extract_between_parentheses:result after pushing item_to_add: `{:?}`\n",
                result
            );
            return extracted;
        }
    }

    let calculation = calculate_expression(input.as_str());
    result.push((input.to_string(), calculation));
    print!("DEBUG:extract_between_parentheses:result: `{:?}`\n", result);
    result

    // match input.find('(') {
    //     Some(start) => match input[start..].rfind(')') {
    //         Some(end) => {
    //             let expr = &input[start + 1..start + end];
    //             print!("DEBUG:extract_between_parentheses:expr: `{}`\n", expr);
    //             let extracted = extract_between_parentheses(expr);
    //             print!(
    //                 "DEBUG:extract_between_parentheses:extracted: `{:?}`\n",
    //                 extracted
    //             );

    // 			let item_to_add = (expr.to_string(), extracted.last().unwrap().1);
    // 			print!("DEBUG:extract_between_parentheses:item_to_add: `{:?}`\n", item_to_add);
    // 			result.push(item_to_add);
    // 			print!("DEBUG:extract_between_parentheses:result after pushing item_to_add: `{:?}`\n", result);
    //             extracted
    //         }
    //         None => panic!("Expression doesn't have a closing parenthesis: `{}`", input),
    //     },
    //     None => {
    //         let calculation = calculate_expression(input);
    //         result.push((input.to_string(), calculation));
    //         print!("DEBUG:extract_between_parentheses:result: `{:?}`\n", result);
    //         result
    //     }
    // }
}

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
