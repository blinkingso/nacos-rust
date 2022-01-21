/// get md5 string with lower case.
pub fn get_md5_string(message: &str) -> String {
    let digest = md5::compute(message.as_bytes());
    format!("{:?}", digest)
}

#[test]
fn test_md5() {
    let digest = get_md5_string("hello world");
    println!("{}", digest);
}
