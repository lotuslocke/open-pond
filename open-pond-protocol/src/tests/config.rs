#[cfg(test)]
use crate::parse_config;

#[test]
fn parse_config_success() {
    let file = String::from("src/tests/test_configs/success.toml");

    let result = parse_config(file);

    println!("{:?}", result);

    assert!(result.is_ok());
}

#[test]
fn parse_config_missing_file() {
    let file = String::from("src/tests/test_configs/missing.toml");

    let result = parse_config(file);

    assert!(result.is_err());
}

#[test]
fn parse_config_bad_format() {
    let file = String::from("src/tests/test_configs/bad_format.toml");

    let result = parse_config(file);

    assert!(result.is_err());
}
