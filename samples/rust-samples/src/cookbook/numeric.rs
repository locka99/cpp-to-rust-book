#[test]
fn convert_number_to_a_string_u32() {
    //
    let value = 17u32;
    let value_as_string = value.to_string();
    //
    assert_eq!("17", value_as_string.as_str());
}

#[test]
fn convert_number_to_a_string_f32() {
    //
    let value = 100.00345f32;
    let value_as_string = value.to_string();
    //
    assert_eq!("100.00345", value_as_string.as_str());
}

#[test]
fn convert_number_to_string_precision() {
    //
    let value = 1234.66667;
    let value_as_string = format!("{:08.2}", value);
    println!("value = {}", value_as_string);
    //
    assert_eq!("01234.67", value_as_string.as_str());
}

#[test]
fn convert_number_to_localized_string() {
    // TODO
}

#[test]
fn string_to_i32() {
    //
    use std::str::FromStr;
    let value_as_str = "12345";
    let value = i32::from_str(value_as_str).unwrap();
    //
    assert_eq!(value, 12345);
}

#[test]
fn convert_numeric_types() {
    //
    let f = 1234.42f32;
    let i = f as i32;
    println!("Value = {}", i);
    //
    assert_eq!(i, 1234);
}
