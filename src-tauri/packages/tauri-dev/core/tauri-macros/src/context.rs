// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::{env::VarError, path::PathBuf};
use syn::{
  parse::{Parse, ParseBuffer},
  punctuated::Punctuated,
  LitStr, PathArguments, PathSegment, Token,
};
use tauri_codegen::{context_codegen, get_config, ContextData};
use tauri_utils::config::parse::does_supported_file_name_exist;

pub(crate) struct ContextItems {
  config_file: PathBuf,
  root: syn::Path,
}

impl Parse for ContextItems {
  fn parse(input: &ParseBuffer<'_>) -> syn::parse::Result<Self> {
    let config_file = if input.is_empty() {
      std::env::var("CARGO_MANIFEST_DIR").map(|m| PathBuf::from(m).join("tauri.conf.json"))
    } else {
      let raw: LitStr = input.parse()?;
      let path = PathBuf::from(raw.value());
      if path.is_relative() {
        std::env::var("CARGO_MANIFEST_DIR").map(|m| PathBuf::from(m).join(path))
      } else {
        Ok(path)
      }
    }
    .map_err(|error| match error {
      VarError::NotPresent => "no CARGO_MANIFEST_DIR env var, this should be set by cargo".into(),
      VarError::NotUnicode(_) => "CARGO_MANIFEST_DIR env var contained invalid utf8".into(),
    })
    .and_then(|path| {
      if does_supported_file_name_exist(&path) {
        Ok(path)
      } else {
        Err(format!(
          "no file at path {} exists, expected tauri config file",
          path.display()
        ))
      }
    })
    .map_err(|e| input.error(e))?;

    let context_path = if input.is_empty() {
      let mut segments = Punctuated::new();
      segments.push(PathSegment {
        ident: Ident::new("tauri", Span::call_site()),
        arguments: PathArguments::None,
      });
      syn::Path {
        leading_colon: Some(Token![::](Span::call_site())),
        segments,
      }
    } else {
      let _: Token![,] = input.parse()?;
      input.call(syn::Path::parse_mod_style)?
    };

    Ok(Self {
      config_file,
      root: context_path,
    })
  }
}

pub(crate) fn generate_context(context: ContextItems) -> TokenStream {
  let context = get_config(&context.config_file)
    .map_err(|e| e.to_string())
    .map(|(config, config_parent)| ContextData {
      dev: cfg!(not(feature = "custom-protocol")),
      config,
      config_parent,
      root: context.root.to_token_stream(),
    })
    .and_then(|data| context_codegen(data).map_err(|e| e.to_string()));

  match context {
    Ok(code) => code,
    Err(error) => quote!(compile_error!(#error)),
  }
}
