mod mod1;
mod mod2;
mod mod3;

fn main() {
    ::hello::mod1::hi();
    hello::mod1::hi();
    mod1::hi();
    mod2::sub_mod::hi();
    mod3::hi();
}

fn foo() {
    println!("crate: [hello]");
}
