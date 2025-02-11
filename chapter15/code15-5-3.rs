#![allow(dead_code)] // 이를 작성하지 않으면 컴파일러가 경고로 화면을 가득 채웁니다.
use rand::{thread_rng, Rng}; // 귀찮다면 rand::*;를 사용해도 됩니다.

#[derive(Debug)]
struct Character {
    strength: u8,       // 힘
    dexterity: u8,      // 민첩성
    constitution: u8,   // 체력
    intelligence: u8,   // 지능
    wisdom: u8,         // 지혜
    charisma: u8,       // 매력(카리스마)
}

enum Dice {
    Three,
    Four,
}

fn roll_dice(dice_choice: &Dice) -> u8 {
    let mut generator = thread_rng(); // 난수 생성기를 만듭니다.
    let mut total = 0;

    match dice_choice {
        Dice::Three => {
            for _ in 0..3 {
                total += generator.gen_range(1..=6);
            }
        }
        Dice::Four => {
            let mut results = vec![]; // 먼저 숫자를 vec에 넣습니다.
            // 앞의 for 루프와 같은 효과를 내지만 다른 방법도 있습니다!
            (0..4).for_each(|_| results.push(generator.gen_range(1..=6)));
            results.sort();
            results.remove(0);
            total += results.into_iter().sum::<u8>();
        }
    }

    total
}

impl Character {
    fn new(dice_choice: Dice) -> Self {
        // 6개의 스탯으로 Vec을 만듭니다.
        let stats = (0..6).map(|_| roll_dice(&dice_choice)).collect::<Vec<u8>>();
        
        // 6개의 인덱스가 모두 있다고 확신하므로 인덱스에 직접 접근하겠습니다.
        Self {
            strength: stats[0],
            dexterity: stats[1],
            constitution: stats[2],
            intelligence: stats[3],
            wisdom: stats[4],
            charisma: stats[5],
        }
    }
}

fn main() {
    let weak_billy = Character::new(Dice::Three);
    let strong_billy = Character::new(Dice::Four);

    println!("{weak_billy:#?}");
    println!("{strong_billy:#?}");
}
