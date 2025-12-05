pub fn generate(module_name: &str) -> String {
    format!(
        r#"[package]
name = "{module_name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]
path = "src/lib.rs"

[dependencies]
kernel = {{ path = "../linux/rust" }}
"#,
        module_name = module_name
    )
}
