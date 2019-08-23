#[macro_use]
extern crate kserd_derive;

use kserd::*;

#[test]
fn struct_with_fields() {
    #[derive(AsKserd)]
    struct Simple {
        a: u8,
        b: String,
    }

    let simple = Simple {
        a: 100,
        b: "Hello, world!".to_string(),
    };

    let kserd = simple.as_kserd();

    println!(
        "{}",
        kserd.as_str_with_config(format::FormattingConfig::default())
    );

    assert_eq!(kserd.id(), Some("Simple"));

    let map = match kserd.val() {
        Value::Cntr(map) => map,
        _ => unreachable!(),
    };

    assert_eq!(map.get(&("a".into())), Some(&(100u64.as_kserd())));
    assert_eq!(map.get(&("b".into())), Some(&("Hello, world!".as_kserd())));
}

#[test]
fn unit_struct() {
    #[derive(AsKserd)]
    struct AUnitStruct;

    let val = AUnitStruct;

    let kserd = val.as_kserd();

    println!(
        "{}",
        kserd.as_str_with_config(format::FormattingConfig::default())
    );

    assert_eq!(kserd.id(), Some("AUnitStruct"));
    assert_eq!(kserd.val(), &Value::Unit);
}
