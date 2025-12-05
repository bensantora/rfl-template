pub fn generate(module_name: &str, author: &str, description: &str, license: &str) -> String {
    format!(
        r#"// SPDX-License-Identifier: {license}

//! {description}

use kernel::prelude::*;

module! {{
    type: {module_struct},
    name: "{module_name}",
    author: "{author}",
    description: "{description}",
    license: "{license}",
}}

struct {module_struct};

impl kernel::Module for {module_struct} {{
    fn init(_module: &'static ThisModule) -> Result<Self> {{
        pr_info!("{module_name}: Module loaded\n");
        Ok({module_struct})
    }}
}}

impl Drop for {module_struct} {{
    fn drop(&mut self) {{
        pr_info!("{module_name}: Module unloaded\n");
    }}
}}
"#,
        module_name = module_name,
        module_struct = to_struct_name(module_name),
        author = author,
        description = description,
        license = license
    )
}

fn to_struct_name(module_name: &str) -> String {
    module_name
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}
