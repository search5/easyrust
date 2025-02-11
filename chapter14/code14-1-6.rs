mod country { // 메인 모드는 pub이 필요하지 않습니다.
    fn print_country(country: &str) { // 참고: 이 함수는 공개되지 않습니다.
        println!("We are in the country of {}", country);
    }

    pub mod province { // 이 mod를 public으로 만듭니다.
        fn print_province(province: &str) { // 참고: 이 함수는 공개되지 않습니다.
            println!("in the province of {}", province);
        }

        pub mod city { // 이 mod를 pubic으로 만듭니다.
            pub fn print_city(country: &str, province: &str, city: &str) {
                // 이 함수는 공개됩니다.

                // 크레이트 레벨에서 country 레벨로 내려갑니다.
                crate::country::print_country(country);
            
                // city에서 province, country로 이동합니다.
                super::super::print_country(country);
                
                crate::country::province::print_province(province);
                super::print_province(province);

                println!("in the city of {}", city);
            }
        }
    }
}

fn main() {
    country::province::city::print_city("Canada", "New Brunswick", "Moncton");
}
