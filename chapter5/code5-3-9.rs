fn main() {
    let my_name = "Mithridates";

    assert_ne!(
        my_name,
        "Mithridates", "You entered {my_name}. Input must not equal Mithridates"
    );
}
