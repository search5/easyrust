enum Bank {
    KookminBANK,
    ShinhanBANK,
    IBKKiupBANK,
    ChungbukBANK
    // 기타 등등...
}

fn get_swift_code(bank: &Bank) -> &'static str {
    use Bank::*;
    match bank {
        KookminBANK => "CZNBKRSE", // 국내 은행의 실제 BIC 코드
        ShinhanBANK => "SHBKKRSE",
        IBKKiupBANK => "IBKOKRSE",
        ChungbukBANK => unreachable!()
    }
}

fn main() {
    // 사용자 입력이 다른 함수에서 이루어진다고 가정합니다.
    // 사용자는 어떤 경우에도 ChungbukBANK를 선택할 수 없습니다.
    let user_input = Bank::IBKKiupBANK;
    println!("{}", get_swift_code(&user_input));
}
