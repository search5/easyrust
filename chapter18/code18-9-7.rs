use std::mem;
use std::ops::{Deref, DerefMut}; // 우리는 이를 사용해 u32의 힘을 얻을 것입니다.

struct Bank {
    // 스마트 포인터 타입입니다. 자체 기본값이 있지만 u32를 사용합니다.
    money_inside: u32,
    money_at_desk: DeskMoney,
}

struct DeskMoney(u32);

impl Default for DeskMoney {
    fn default() -> Self {
        Self(50) // 기본값이 항상 (0이 아닌) 50입니다.
    }
}

impl Deref for DeskMoney { // 이를 통해 *를 사용해 u32에 액세스할 수 있습니다.
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DeskMoney { // 이를 통해 더하기, 빼기 등을 할 수 있습니다.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Bank {
    fn check_money(&self) {
        println!(
            "There is ${} in the back and ${} at the desk.\n",
            self.money_inside, *self.money_at_desk // *를 사용하면 u32를 출력할 수 있습니다.
        );
    }
}

struct Robber {
    money_in_pocket: u32,
}

impl Robber {
    fn check_money(&self) {
        println!("The robber has ${} right now.\n", self.money_in_pocket);
    }

    fn rob_bank(&mut self, bank: &mut Bank) {
        // 여기서는 돈을 가져가고 기본값이므로 50을 남깁니다.
        let new_money = mem::take(&mut bank.money_at_desk);
        
        // u32만 추가할 수 있으므로 *를 사용합니다.
        // DeskMoney는 여기에 더할 수 없습니다.
        self.money_in_pocket += *new_money;

        // money_in_pocket과 동일합니다.
        bank.money_inside -= *new_money;

        println!("She robbed the bank. She now has ${}!\n", self.money_in_pocket);
    }
}

fn main() {
    let mut bank_of_klezkavania = Bank { // 은행 설정
        money_inside: 5000,
        money_at_desk: DeskMoney(50),
    };

    bank_of_klezkavania.check_money();
    
    let mut robber = Robber { // 강도 설정
        money_in_pocket: 50,
    };

    robber.check_money();

    robber.rob_bank(&mut bank_of_klezkavania); // 강도가 돈을 확인합니다.
    robber.check_money();
    bank_of_klezkavania.check_money();

    robber.rob_bank(&mut bank_of_klezkavania); // 다시 돈을 확인합니다.
    robber.check_money();
    bank_of_klezkavania.check_money();
}
