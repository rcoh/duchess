use duchess::{java, Global, JvmOp, ToJava};
use std::collections::HashMap;

#[test]
fn test_hashmap_roundtrip() {
    let mut test_map = HashMap::new();
    test_map.insert("a".to_string(), "abc".to_string());
    test_map.insert("b".to_string(), "cde".to_string());

    let java: Global<java::util::HashMap<java::lang::String, java::lang::String>> = test_map
        .to_java::<java::util::HashMap<java::lang::String, java::lang::String>>()
        .global()
        .execute()
        .unwrap()
        .unwrap();
    assert_eq!(
        java.get("a").to_rust().execute().unwrap(),
        Some("abc".to_string())
    );
    assert_eq!(
        java.get("b").to_rust().execute().unwrap(),
        Some("cde".to_string())
    );
}
