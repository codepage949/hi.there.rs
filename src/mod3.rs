pub fn hi() {
    crate::foo();
    // it is possible to reference '(lib)hello because mod3 is not being used in (lib)hello.'
    hello::mod1::hi();
    hello::mod2::sub_mod::hi();
    println!("mod3: hi^0^v");
}
