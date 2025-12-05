pub fn generate(module_name: &str) -> String {
    format!("obj-m := {}.o\n", module_name)
}
