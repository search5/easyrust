// 🚧
let mut x = 10;

for i in 0..10 { // 스레드 1
    x += 1;
}

for i in 0..10 { // 스레드 2
    x += 1;
}
