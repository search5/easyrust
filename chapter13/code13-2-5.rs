// 🚧
impl Deref for HoldsANumber {
    type Target = u8;
    // 이것이 '연관 타입'임을 기억하세요.
    // 연관 타입은 함께하는 타입입니다.
    // 올바른 타입의 Target(반환하려는 타입)을 사용해야 합니다.
    
    fn deref(&self) -> &Self::Target {
        // 러스트는 *를 사용할 때 .deref()를 호출합니다.
        // 방금 Target을 u8로 정의했기 때문에 이해하기 쉽습니다.
        &self.0
        // 튜플 구조이기 때문에 &self.0을 선택했습니다.
        // 명명된 구조체에서는 "&self.number"와 같습니다.
    }
}
