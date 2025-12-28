use cosy::{from_str, schema};

#[test]
fn test_optional_field_support() {
    // Schema with an optional string field
    let schema_text = r#"{
        required_field: "string"
        optional_field: { type: "string", optional: true }
    }"#;
    let schema = from_str(schema_text).unwrap();

    // Case 1: Field is missing (Should Pass)
    let config_missing = from_str(r#"{ required_field: "present" }"#).unwrap();
    let report = schema::validate(&config_missing, &schema).unwrap();
    assert!(
        report.is_empty(),
        "Optional field should be allowed to be missing"
    );

    // Case 2: Field is present and valid (Should Pass)
    let config_present =
        from_str(r#"{ required_field: "present", optional_field: "here" }"#).unwrap();
    let report_2 = schema::validate(&config_present, &schema).unwrap();
    assert!(
        report_2.is_empty(),
        "Optional field should validate when present"
    );

    // Case 3: Field is present but invalid type (Should Fail)
    let config_invalid = from_str(r#"{ required_field: "present", optional_field: 123 }"#).unwrap();
    let report_3 = schema::validate(&config_invalid, &schema).unwrap();
    assert!(
        !report_3.is_empty(),
        "Optional field should still check type when present"
    );
}
