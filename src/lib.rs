// SPDX-FileCopyrightText: Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

// include the generated protobuf source as a submodule
#[allow(clippy::all)]
pub mod protobuf {
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }
    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
}

#[cfg(test)]
mod tests {
    use crate::protobuf::expression::literal::LiteralType;
    use crate::protobuf::expression::Literal;

    #[test]
    fn literal() {
        let _ = Literal {
            nullable: true,
            literal_type: Some(LiteralType::I32(123)),
        };
    }
}
