use std::env;

fn main() {
    if let Ok(target_env) = env::var("TARGET_ENV") {
        match target_env.as_str() {
            "x86_64-apple-ios-macabi" => {
                println!("cargo:rustc-cfg=mac_catalyst");
            }
            _ => {}
        }
    }
}
