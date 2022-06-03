// SPDX-License-Identifier: Apache-2.0

use glob::glob;

fn main() -> Result<(), String> {
    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    let root = "substrait/proto";
    let mut relative_paths = vec![];
    let proto_paths =
        glob(&format!("{}/**/*.proto", root)).map_err(|e| format!("glob error: {}", e))?;
    for path in proto_paths {
        let path = path
            .map(|p| {
                p.to_str()
                    .unwrap_or_else(|| panic!("Path is not valid Unicode"))
                    .to_string()
            })
            .map_err(|e| format!("glob error: {}", e))?;
        if let Some(pos) = path.find(root) {
            let path = &path[pos + root.len() + 1..];
            relative_paths.push(path.to_string());
        }
    }

    for path in &relative_paths {
        println!("cargo:rerun-if-changed=substrait/proto/{}", path);
    }

    prost_build::compile_protos(&relative_paths, &[root])
        .map_err(|e| format!("protobuf compilation failed: {}", e))
}
