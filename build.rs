fn main() {
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=assets/templates");
    println!("cargo:rerun-if-changed=assets/static");
}
