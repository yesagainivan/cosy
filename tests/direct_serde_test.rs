use cosy::serde::from_value;
use cosy::value::{Value, ValueKind};
use serde::Deserialize;

#[test]
fn test_from_value_direct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        port: u16,
    }

    // Manually create a Value
    let mut map = indexmap::IndexMap::new();
    map.insert("port".to_string(), Value::integer(8080));
    let value = Value::object(map);

    // Deserialize directly from Value
    let config: Config = from_value(value).unwrap();

    assert_eq!(config.port, 8080);
}
