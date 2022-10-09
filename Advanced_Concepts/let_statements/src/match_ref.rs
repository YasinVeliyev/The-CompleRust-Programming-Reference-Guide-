struct Person(String);

pub fn match_ref() {
    let a = Person("Yasin Vəliyev".to_owned());
    match a {
        Person(ref a) => println!("{}", a),
        _ => panic!("Oh no !"),
    };
    let b = a;
}
