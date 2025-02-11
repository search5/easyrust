fn main() {
    let months = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    let filtered_months = months
        .into_iter()                         // 이터레이터를 만듭니다.
        .filter(|month| month.len() < 5)     // 5바이트보다 긴 달을 원하지 않습니다.
                                             // 각 문자가 1바이트이므로
                                             // .len()을 사용하면 됩니다.
        .filter(|month| month.contains("u")) // 또한 문자 u가 있는 달만 원합니다.
        .collect::<Vec<&str>>();
    
    println!("{:?}", filtered_months);
}
