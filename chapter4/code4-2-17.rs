fn main() {
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"]
    ];
    
    for mut city in weather_vec {
        // 데이터에서 모든 첫 번째 항목은 도시 이름입니다.
        println!("For the city of {}:", city[0]);

        while let Some(information) = city.pop() {
            // 더 이상 꺼낼 수 없을 때까지 계속 진행하세요.
            // 벡터의 항목 수가 0개에 도달하면 None을 반환하고 멈춥니다.
            if let Ok(number) = information.parse::<i32>() {
                // information이라는 변수를 i32로 바꾸려고 시도합니다.
                // 반환된 결과가 Ok(숫자)면 출력합니다.
                println!("The number is: {number}");
            }
            // 오류가 발생하면 아무것도 하지 않으므로 아무것도 쓰지 않습니다.
            // 나머지는 모두 버립니다.
        }
    }
}
