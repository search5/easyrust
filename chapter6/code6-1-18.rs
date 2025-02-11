use std::fmt::Debug; // 이제 매번 std::fmt::Debug를 쓸 필요가 없습니다.

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait Magic {} // 이러한 트레이트에 대한 메서드가 없습니다.
               // 이들은 단지 트레이트 바운드입니다.

trait FightClose {}

trait FightFromDistance {}

impl FightClose for Ranger {} // 각 타입은 FightClose를 얻습니다.

impl FightClose for Wizard {}

impl FightFromDistance for Ranger {} // 그러나 Ranger만이 FightFromDistance를 얻습니다.

impl Magic for Wizard {} // 오직 Wizard만이 Magic(마법)을 얻습니다.

fn attack_with_bow<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent now has {} health left. \
You are now at: {:?}",
            opponent.health, character
        );
    }
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. \
You are now at: {:?}",
        opponent.health, character
    );
}

fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You raise your hands and cast a fireball! \
Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, character
        );
    }
}

fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };
    
    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fireball(&radagast, &mut uruk_hai, 8);
}
