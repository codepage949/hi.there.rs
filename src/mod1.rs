pub fn hi() {
    crate::foo();
    // if you want to reference (lib)hello, should use 'crate' instead of 'hello'.
    // because mod1 is being used in (lib)hello.
    // hello::mod2::sub_mod::hi();
    println!("mod1: hi^0^v");
}
