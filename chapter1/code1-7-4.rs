fn main() {
    let naver_base_url = "naver";
    let google_base_url = "google";
    let microsoft_base_url = "microsoft";
    // 이 방법은 보기 싫고 읽기 어렵습니다.
    println!("The url is www.{naver_base_url}.com");
    println!("The url is www.{google_base_url}.com");
    println!("The url is www.{microsoft_base_url}.com");

    // 이렇게 하면 훨씬 더 깔끔하게 정렬됩니다.
    println!("The url is www.{}.com", naver_base_url);
    println!("The url is www.{}.com", google_base_url);
    println!("The url is www.{}.com", microsoft_base_url);
}
