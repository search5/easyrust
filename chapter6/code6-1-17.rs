struct Monster {
    health: i32,
}

#[derive(Debug)] // Wizard에 Debug를 추가하고,
struct Wizard {
    health: i32, // health를 추가했습니다.
}

#[derive(Debug)] // Ranger에도 추가했습니다.
struct Ranger {
    health: i32, // Ranger에도 추가했습니다.
}

trait FightClose: std::fmt::Debug {
    // 타입이 FightClose를 사용하려면 Debug가 필요합니다.
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent now has {} health left. \
You are now at: {:?}",
            // Debug가 있으므로 이제 {:?}로 self를 출력할 수 있습니다.
            opponent.health,
            &self
        );
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your hand. Your opponent now has {} health left. \
You are now at: {:?}",
            opponent.health, &self
        );
    }
}

impl FightClose for Wizard {}

impl FightClose for Ranger {}

trait FightFromDistance: std::fmt::Debug {
    // FightClose는 Debug가 필요하기 때문에
    // FightFromDistance: FightClose 트레이트를 수행할 수도 있습니다.
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow. Your opponent now has {} health left. \
You are now at: {:?}",
                opponent.health, self
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }

        println!(
            "You attack with your rock. Your opponent now has {} health left. \
You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightFromDistance for Ranger {}

fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);
}
