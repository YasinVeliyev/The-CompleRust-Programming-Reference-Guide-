fn give_me<T>(value: T) {
    let _ = value;
}

pub fn _func() {
    give_me("value");
    give_me(1024);
}
