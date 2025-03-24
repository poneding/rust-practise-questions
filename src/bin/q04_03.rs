/// # Chapter 4 - Enum & Patterns
///
/// Create a calculator with the help of an enum named `Operation` that as values such as `Add, Sub, Div, Multi`.
/// `hint:` For input like `2+5*10` it should first evaluate `5*10` and then add 2 to it.
fn main() {
    let expression = "2+5*10";
    assert_eq!(parse_eval(expression), 52.0);

    let expression = "2+3*4-15/3";
    assert_eq!(parse_eval(expression), 9.0);

    let expression = "10-2/2";
    assert_eq!(parse_eval(expression), 9.0);
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Div,
    Multi,
}

fn calculate(op: Operation, n1: f64, n2: f64) -> f64 {
    match op {
        Operation::Add => n1 + n2,
        Operation::Sub => n1 - n2,
        Operation::Div => n1 / n2,
        Operation::Multi => n1 * n2,
    }
}

fn parse_eval(expression: &str) -> f64 {
    let mut nums = vec![];
    let mut ops = vec![];

    let mut curr_num = String::new();
    for c in expression.chars() {
        if c.is_digit(10) || c == '.' {
            curr_num.push(c);
        } else {
            match c {
                '+' => ops.push(Operation::Add),
                '-' => ops.push(Operation::Sub),
                '*' => ops.push(Operation::Multi),
                '/' => ops.push(Operation::Div),
                _ => panic!("Invalid operator found in expression: {}", c),
            }
            // 将当前数字加入到 nums 中
            nums.push(curr_num.parse::<f64>().unwrap());
            curr_num.clear();
        }
    }

    // 将最后一个数字加入到 nums 中
    nums.push(curr_num.parse().unwrap());

    let mut i = 0;
    while i < ops.len() {
        if ops[i] == Operation::Multi || ops[i] == Operation::Div {
            let result = calculate(ops[i], nums[i], nums[i + 1]);
            nums[i] = result;
            nums.remove(i + 1);
            ops.remove(i);
        } else {
            i += 1;
        }
    }

    let mut result = nums[0];
    for i in 0..ops.len() {
        result = calculate(ops[i], result, nums[i + 1]);
    }

    result
}
