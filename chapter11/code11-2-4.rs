use std::fmt::Display;

fn prints_it_impl_trait(input: impl Display) {
    println!("You can print many things, including {input}");
}

fn prints_it_regular_generic<T: Display>(input: T) {
    println!("You can print many things, including {input}");
}

fn main() {
    prints_it_regular_generic::<u8>(100); // 원한다면 u8을 지정할 수 있습니다.
    prints_it_impl_trait(100); // 여기서는 타입을 지정할 수 없으며, i32로 추론됩니다.
    // prints_it_impl_trait::<u8>(100);은 동작하지 않습니다.
}
