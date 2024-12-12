use serde_jsonc2::Value;

#[test]
fn test() {
    let x1 = serde_jsonc2::from_str::<Value>("18446744073709551615.");
    assert!(x1.is_err());
    let x2 = serde_jsonc2::from_str::<Value>("18446744073709551616.");
    assert!(x2.is_err());
}
