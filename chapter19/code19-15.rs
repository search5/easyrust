macro_rules! dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // 임시 변수의 수명을 제어하여 값이 안전하게 사용되도록 match를 사용합니다.
        // 참고: https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // 단일 인수가 있는 후행 쉼표는 무시됩니다.
    ($val:expr,) => { dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}
