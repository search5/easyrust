use std::thread;

fn main() {
    thread::scope(|scope| {
        // 여기서는 scope라고 부르지만 다른 이름을 사용할 수도 있습니다.
        scope.spawn(|| {
            // 스레드 작업 수행
        });

        scope.spawn(|| {
            // 더 많은 스레드 작업 수행
        });
    }); // 스레드는 여기에서 자동으로 join 됩니다.
}
