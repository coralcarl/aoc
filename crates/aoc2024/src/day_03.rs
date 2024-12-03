struct Token {
    op: String,
    values: Vec<u64>,
}

fn tokenizer(input: &str, valid_tokens: Vec<&str>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let mut token: Option<&str> = None;
        for valid in &valid_tokens {
            if i + valid.len() <= input.len() && input[i..i + valid.len()] == **valid {
                token = Some(valid);
                i += valid.len();
                break;
            }
        }

        if token.is_some() {
            let mut values: Vec<u64> = Vec::new();
            let mut current = String::new();

            let mut valid_open = false;
            let mut valid_close = false;

            for (j, c) in input[i..].chars().enumerate() {
                match c {
                    '(' if !valid_open => valid_open = true,
                    ',' if valid_open && !current.is_empty() => {
                        values.push(current.parse::<u64>().unwrap());
                        current = String::new();
                    }
                    '0'..='9' if valid_open => {
                        current.push(c);
                    }
                    ')' if valid_open => {
                        if !current.is_empty() {
                            values.push(current.parse::<u64>().unwrap());
                        }

                        valid_close = true;
                        i += j;
                        break;
                    }
                    _ => {
                        i += j;
                        i -= 1;
                        break;
                    }
                }
            }

            if valid_close {
                match token {
                    Some("mul") if values.len() == 2 => {
                        tokens.push(Token {
                            op: token.unwrap().to_string(),
                            values,
                        });
                    }
                    Some("do") | Some("don't") if values.is_empty() => {
                        tokens.push(Token {
                            op: token.unwrap().to_string(),
                            values,
                        });
                    }
                    _ => {}
                }
            }
        }

        i += 1;
    }

    tokens
}

pub fn part1(input: &str) -> String {
    tokenizer(&input, vec!["mul"])
        .iter()
        .map(|tk| tk.values[0] * tk.values[1])
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut enabled = true;
    let mut result = 0;

    for tk in tokenizer(&input, vec!["mul", "don't", "do"]) {
        match tk.op.as_str() {
            "mul" if enabled => {
                result += tk.values[0] * tk.values[1];
            }
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {}
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "xmul(2,4)%&mul[3,iw!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&input), "161");
    }

    #[test]
    fn example2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&input), "48");
    }
}
