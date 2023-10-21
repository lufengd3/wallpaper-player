// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
  helpers::updater_signature::{generate_key, save_keypair},
  Result,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(about = "Generate keypair to sign files")]
pub struct Options {
  /// Set private key password when signing
  #[clap(short, long)]
  password: Option<String>,
  /// Write private key to a file
  #[clap(short, long)]
  write_keys: Option<PathBuf>,
  /// Overwrite private key even if it exists on the specified path
  #[clap(short, long)]
  force: bool,
}

pub fn command(options: Options) -> Result<()> {
  if options.password.is_none() {
    println!("Generating new private key without password.")
  }
  let keypair = generate_key(options.password).expect("Failed to generate key");

  if let Some(output_path) = options.write_keys {
    let (secret_path, public_path) =
      save_keypair(options.force, output_path, &keypair.sk, &keypair.pk)
        .expect("Unable to write keypair");

    println!(
        "\nYour keypair was generated successfully\nPrivate: {} (Keep it secret!)\nPublic: {}\n---------------------------",
        secret_path.display(),
        public_path.display()
        )
  } else {
    println!(
      "\nYour secret key was generated successfully - Keep it secret!\n{}\n\n",
      keypair.sk
    );
    println!(
          "Your public key was generated successfully:\n{}\n\nAdd the public key in your tauri.conf.json\n---------------------------\n",
          keypair.pk
        );
  }

  println!("\nEnvironment variables used to sign:\n`TAURI_PRIVATE_KEY`  Path or String of your private key\n`TAURI_KEY_PASSWORD`  Your private key password (optional)\n\nATTENTION: If you lose your private key OR password, you'll not be able to sign your update package and updates will not work.\n---------------------------\n");

  Ok(())
}
