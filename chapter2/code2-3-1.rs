fn main() {
    // 한국 이름은 문제없이 사용할 수 있습니다. &str은 UTF-8이기 때문입니다.
    let name = "자우림";
    // Ț와 ș는 UTF-8에서 문제가 되지 않습니다.
    let other_name = String::from("Adrian Fahrenheit Țepeș");
}
