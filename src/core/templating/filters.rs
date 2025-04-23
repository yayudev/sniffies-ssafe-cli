use std::collections::HashMap;

use convert_case::{Case, Casing};
use tera::{Result, Value, to_value, try_get_value};

pub fn pascal_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let value = try_get_value!("pascal_filter", "value", String, value);
    let value_parsed = value.to_case(Case::UpperCamel);
    Ok(to_value(value_parsed).unwrap())
}

pub fn kebab_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let value = try_get_value!("kebab_filter", "value", String, value);
    let value_parsed = value.to_case(Case::Kebab);
    Ok(to_value(value_parsed).unwrap())
}

pub fn constant_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let value = try_get_value!("kebab_filter", "value", String, value);
    let value_parsed = value.to_case(Case::Constant);
    Ok(to_value(value_parsed).unwrap())
}

pub fn camel_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let value = try_get_value!("kebab_filter", "value", String, value);
    let value_parsed = value.to_case(Case::Camel);
    Ok(to_value(value_parsed).unwrap())
}

#[test]
pub fn test_pascal_filter() {
    let inputs = vec![
        (r#"cruiseThisArea"#, r#"CruiseThisArea"#),
        (r#"CruiseThisArea"#, r#"CruiseThisArea"#),
        (r#"cruise-this-area"#, r#"CruiseThisArea"#),
    ];

    for (input, expected) in inputs {
        let value = to_value(input).unwrap();
        let result = pascal_filter(&value, &HashMap::new());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

#[test]
pub fn test_kebab_filter() {
    let inputs = vec![
        (r#"cruiseThisArea"#, r#"cruise-this-area"#),
        (r#"CruiseThisArea"#, r#"cruise-this-area"#),
        (r#"cruise-this-area"#, r#"cruise-this-area"#),
    ];

    for (input, expected) in inputs {
        let value = to_value(input).unwrap();
        let result = kebab_filter(&value, &HashMap::new());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

#[test]
pub fn test_constant_filter() {
    let inputs = vec![
        (r#"cruiseThisArea"#, r#"CRUISE_THIS_AREA"#),
        (r#"CruiseThisArea"#, r#"CRUISE_THIS_AREA"#),
        (r#"cruise-this-area"#, r#"CRUISE_THIS_AREA"#),
    ];

    for (input, expected) in inputs {
        let value = to_value(input).unwrap();
        let result = constant_filter(&value, &HashMap::new());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

#[test]
pub fn test_camel_filter() {
    let inputs = vec![
        (r#"cruiseThisArea"#, r#"cruiseThisArea"#),
        (r#"CruiseThisArea"#, r#"cruiseThisArea"#),
        (r#"cruise-this-area"#, r#"cruiseThisArea"#),
    ];

    for (input, expected) in inputs {
        let value = to_value(input).unwrap();
        let result = camel_filter(&value, &HashMap::new());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
