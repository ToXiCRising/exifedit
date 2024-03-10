
fn main() {
    println!("cargo:rustc-link-arg=resources/resources.res");
    slint_build::compile("ui/appwindow.slint").unwrap();   
}
