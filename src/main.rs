use shadow_rs::shadow;

shadow!(build);

fn main() {
    println!("Hello, shadow! {}", build::PKG_VERSION);
}
