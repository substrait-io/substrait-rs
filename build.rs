// SPDX-FileCopyrightText: Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

fn main() -> Result<(), String> {
    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    let proto_defs = [
        "substrait/algebra.proto",
        "substrait/capabilities.proto",
        "substrait/extensions/extensions.proto",
        "substrait/function.proto",
        "substrait/parameterized_types.proto",
        "substrait/plan.proto",
        "substrait/type_expressions.proto",
        "substrait/type.proto",
    ];

    for def in &proto_defs {
        println!("cargo:rerun-if-changed=substrait/proto/{}", def);
    }

    let paths: Vec<String> = proto_defs
        .iter()
        .map(|s| format!("substrait/proto/{}", s))
        .collect();

    prost_build::compile_protos(&paths, &["substrait/proto"])
        .map_err(|e| format!("protobuf compilation failed: {}", e))
}
