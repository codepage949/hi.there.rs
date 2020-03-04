pub fn hi() {
    crate::foo();
    crate::mod4::sub_mod::dz();
    println!("mod4::sub_mod: hi^0^v");
}

fn dz() {
    println!("dz");
}
