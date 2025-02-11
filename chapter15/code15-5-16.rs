trait SaysHello {
    fn hello(&self) {
        println!("Hello");
    }
}

impl<T> SaysHello for T {}
