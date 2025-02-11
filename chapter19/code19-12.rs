macro_rules! todo {
    () => (panic!("not yet implemented"));
    ($($arg:tt)+) => (panic!("not yet implemented: {}", $crate::format_args!($($arg)+)));
}
