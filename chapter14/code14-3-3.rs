const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if !input.chars().all(|character| OKAY_CHARACTERS.contains(character)) ||
    !input.chars().take(2).any(|character| character.is_numeric())
    {
        panic!("Please only input numbers, +-, or spaces.");
    }

    let input = input.trim_end_matches(|x| "+- ".contains(x)).chars().filter(|x| *x != ' ').collect::<String>(); // 끝에 +와 -를 제거하고 모든 공백을 제거합니다.
    let mut result_vec = vec![]; // 결과는 여기로 들어갑니다.
    let mut push_string = String::new(); // 우리가 매번 밀어 넣는 문자열입니다. 루프에서 계속 재사용합니다.
    
    for character in input.chars() {
        match character {
            '+' => {
                if !push_string.is_empty() {
                `    // 문자열이 비었으면 ""를 result_vec에 밀어 넣지 않습니다.
                    // 비어 있지 않다면 숫자가 되므로 이를 vec에 밀어 넣습니다.
                    result_vec.push(push_string.clone());
                    push_string.clear(); // 그런 다음 문자열을 지웁니다.`
                }
            },
            '-' => { // -를 받으면,
                if push_string.contains('-') || push_string.is_empty() {
                    // 비었는지 확인하고 비어 있지 않으면 -가 있는지 확인합니다.
                    push_string.push(character) // 그렇다면 밀어 넣습니다.
                } else { // 그렇지 않으면 숫자가 포함됩니다.
                    result_vec.push(push_string.clone());

                    // 따라서 result_vec에 숫자를 밀어 넣고 지운 다음 -를 밀어 넣습니다.
                    push_string.clear();
                    push_string.push(character);
                }
            },
            number => {
                // number는 ‘일치하는 다른 모든 것’을 의미합니다.
                // 여기에서 이름을 선택했습니다.
                if push_string.contains('-') {
                    // 먼저 밀어 넣을 문자가 있을 수 있습니다.
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(number);
                } else {
                    // 하지만 그렇지 않다면 number를 밀어 넣을 수 있다는 뜻입니다.
                    push_string.push(number);
                }
            },
        }
    }

    result_vec.push(push_string); // 루프가 끝난 후 마지막으로 한 번 더 밀어 넣습니다.
    // 더 이상 사용하지 않으니 .clone()이 필요하지 않습니다.

    let mut total = 0; // 이제 수학 연산을 할 시간입니다. total을 0으로 초기화합니다.
    let mut adds = true; // true = 더하기, false = 빼기
    let mut math_iter = result_vec.into_iter();
    
    while let Some(entry) = math_iter.next() { // 항목을 반복합니다.
        if entry.contains('-') {
            // - 문자가 있으면 짝수인지 홀수인지 확인합니다.
            if entry.chars().count() % 2 == 1 {
                adds = match adds {
                    true => false,
                    false => true
                };
                continue; // 다음 항목으로 이동합니다.
            } else {
                continue;
            }
        }

        if adds == true {
            total += entry.parse::<i32>().unwrap(); // '-'가 없으면 숫자여야 합니다.
            // 따라서 unwrap을 호출해도 안전합니다.
        } else {
            total -= entry.parse::<i32>().unwrap();
            adds = true; // 빼고 나면 adds는 true로 설정합니다.
        }
    }
    total // 마지막으로 total을 반환합니다.
}

/// 다음을 확인하기 위해 몇 가지 테스트를 추가하겠습니다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }
    
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }
    
    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - -1"), 2);
    }
    
    #[test]
    fn nine_plus_nine_minus_nine_minus_nine_is_zero() {
        assert_eq!(math("9+9-9-9"), 0); // 새로운 테스트입니다.
    }
    
    #[test]
    fn eight_minus_nine_plus_nine_is_eight_even_with_characters_on_the_end() {
        assert_eq!(math("8 - 9    +9-----+++++"), 8); // 새로운 테스트입니다.
    }
    
    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + seven");
    }
}