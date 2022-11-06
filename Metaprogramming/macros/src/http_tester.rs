#[macro_export]
macro_rules! http_test {
    ($url:tt GET => $code:expr) => {
        let request = reqwest::blocking::get($url).unwrap();
        println!("Testing GET {} => {}", $url, $code);
        assert_eq!(request.status().as_u16(), $code);
    };
    ($url:tt POST => $code:expr,$($k:expr => $v:expr),*) => {
        let params = [$(($k,$v),)*];
        println!("{:#?}",params);
        let client=reqwest::blocking::Client::new();
        let res=client.post($url).form(&params).send().unwrap();
        println!("Testing POST {} => {}", $url, $code);
        assert_eq!(res.status().as_u16(), $code);
    };
}

pub fn http_test() {
    http_test!("https://duckduckgo.com/" GET=>200);
    http_test!("https://duckduckgo.com/" POST=>200,"hello"=>"world","foo"=>"bar");
}
