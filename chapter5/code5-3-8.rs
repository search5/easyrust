fn main() {
    let my_name = "Loki Laufeyson";

    assert!(
        my_name == "Loki Laufeyson",
        "{my_name} should be Loki Laufeyson"
    );
    assert_eq!(
        my_name, "Loki Laufeyson",
        "{my_name} and Loki Laufeyson should be equal"
    );
    assert_ne!(
        my_name,
        "Mithridates", "You entered {my_name}. Input must not equal Mithridates"
    );
}
