// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() {
  let mut codegen = tauri_build::CodegenContext::new();
  if !cfg!(feature = "custom-protocol") {
    codegen = codegen.dev();
  }
  codegen.build();
  tauri_build::build()
}
