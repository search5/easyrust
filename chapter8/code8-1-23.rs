fn main() {
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>();
    // 이는 Vec<usize, &str> 타입이지만,
    // 타입 추론을 통해 러스트가 알아서 처리하도록 할 수 있습니다.
    println!("{:?}", rule_locations);
}
