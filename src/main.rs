fn main() {
    let program = parse_from_japanese("3を、4で、2を、割ったので、引く".to_string());
    dbg!(program.clone(), kaku(program));
}

fn kaku(source: String) -> Option<f64> {
    let mut way_stack: Vec<f64> = vec![];
    let mut obj_stack: Vec<f64> = vec![];
    let tokens = source.split_whitespace();
    for token in tokens {
        let token: Vec<&str> = token.split(":").collect();
        if token.len() == 2 {
            let (token, case) = (
                token.get(0)?.to_string(),
                token.get(1)?.chars().collect::<Vec<char>>(),
            );
            if case.get(0)? == &'a' {
                match token.as_str() {
                    "add" => {
                        let a = obj_stack.pop()?;
                        let b = way_stack.pop()?;
                        let result = a + b;
                        if let Some('w') = case.get(1) {
                            way_stack.push(result)
                        } else {
                            obj_stack.push(result);
                        }
                    }
                    "sub" => {
                        let a = obj_stack.pop()?;
                        let b = way_stack.pop()?;
                        let result = a - b;
                        if let Some('w') = case.get(1) {
                            way_stack.push(result)
                        } else {
                            obj_stack.push(result);
                        }
                    }
                    "mul" => {
                        let a = obj_stack.pop()?;
                        let b = way_stack.pop()?;
                        let result = a * b;
                        if let Some('w') = case.get(1) {
                            way_stack.push(result)
                        } else {
                            obj_stack.push(result);
                        }
                    }
                    "div" => {
                        let a = obj_stack.pop()?;
                        let b = way_stack.pop()?;
                        let result = a / b;
                        if let Some('w') = case.get(1) {
                            way_stack.push(result)
                        } else {
                            obj_stack.push(result);
                        }
                    }
                    _ => return None,
                }
            } else if case.get(0)? == &'w' {
                way_stack.push(token.parse().unwrap_or(0.0));
            } else if case.get(0)? == &'o' {
                obj_stack.push(token.parse().unwrap_or(0.0));
            } else {
                return None;
            }
        } else {
            obj_stack.push(token.get(0)?.parse().unwrap_or(0.0));
        }
        // dbg!(&obj_stack, &way_stack);
    }
    obj_stack.pop()
}

fn parse_from_japanese(source: String) -> String {
    source
        .replace("足したので", "add:aw")
        .replace("足したのを", "add:a")
        .replace("足す", "add:a")
        .replace("引いたので", "sub:aw")
        .replace("引いたのを", "sub:a")
        .replace("引く", "sub:a")
        .replace("掛けたので", "mul:aw")
        .replace("掛けたのを", "mul:a")
        .replace("掛ける", "mul:a")
        .replace("割ったので", "div:aw")
        .replace("割ったのを", "div:a")
        .replace("割る", "div:a")
        .replace("、", " ")
        .replace("を", ":o")
        .replace("で", ":w")
}
