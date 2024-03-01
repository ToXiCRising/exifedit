
fn main() {
    println!("cargo:rustc-link-arg=resources.res");
    slint_build::compile("ui/appwindow.slint").unwrap();   
}
