use crate::input::analyse;
use crate::SourceCode;

#[test]
fn test_example_rs() {
    let file = SourceCode::from_path("../../examples/test.rs").unwrap();
    let res = analyse(&file);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    println!("{}", res.unwrap());
}

#[test]
fn test_small_rs() {
    let file = SourceCode::from_path("../../examples/small.rs").unwrap();
    let res = analyse(&file);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    println!("{}", res.unwrap());
}
