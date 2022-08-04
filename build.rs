// build.rs
fn main() {
    cc::Build::new()
                .file("src/c/implode.c")
                .include("src")
                .compile("implode.a");
}