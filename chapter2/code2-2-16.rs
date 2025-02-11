fn main() {
    let title = "TODAY'S NEWS";
    // 변수 이름 없음, -로 채움, 30자 길이로 중앙에 배치
    println!("{:-^30}", title);
    let bar = "|";
    // 변수 이름 없음, 15자씩 왼쪽과 오른쪽에 공백으로 채움
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    // 변수 이름 city1 및 city2, 각각 왼쪽과 오른쪽에 -로 채움
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
