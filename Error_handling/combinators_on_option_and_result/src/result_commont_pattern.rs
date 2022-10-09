use std::string::FromUtf8Error;

pub fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err),
    };
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

pub fn str_upper_match_with_question_operator(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str).map(|s| s.to_uppercase())?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}
