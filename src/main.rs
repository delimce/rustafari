mod modules {
    pub mod initial;
}

fn main() {
    modules::initial::loader::load();
    // other code...
}   