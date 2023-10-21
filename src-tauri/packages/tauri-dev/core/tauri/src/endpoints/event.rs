// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused_imports)]

use super::InvokeContext;
use crate::{
  api::ipc::CallbackFn,
  event::is_event_name_valid,
  event::{listen_js, unlisten_js},
  runtime::window::is_label_valid,
  sealed::ManagerBase,
  Manager, Runtime,
};
use serde::{de::Deserializer, Deserialize};
use serde_json::Value as JsonValue;
use tauri_macros::{command_enum, CommandModule};

pub struct EventId(String);

impl<'de> Deserialize<'de> for EventId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let event_id = String::deserialize(deserializer)?;
    if is_event_name_valid(&event_id) {
      Ok(EventId(event_id))
    } else {
      Err(serde::de::Error::custom(
        "Event name must include only alphanumeric characters, `-`, `/`, `:` and `_`.",
      ))
    }
  }
}

pub struct WindowLabel(String);

impl<'de> Deserialize<'de> for WindowLabel {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let event_id = String::deserialize(deserializer)?;
    if is_label_valid(&event_id) {
      Ok(WindowLabel(event_id))
    } else {
      Err(serde::de::Error::custom(
        "Window label must include only alphanumeric characters, `-`, `/`, `:` and `_`.",
      ))
    }
  }
}

/// The API descriptor.
#[command_enum]
#[derive(Deserialize, CommandModule)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  /// Listen to an event.
  #[serde(rename_all = "camelCase")]
  Listen {
    event: EventId,
    window_label: Option<WindowLabel>,
    handler: CallbackFn,
  },
  /// Unlisten to an event.
  #[serde(rename_all = "camelCase")]
  Unlisten { event: EventId, event_id: u64 },
  /// Emit an event to the webview associated with the given window.
  /// If the window_label is omitted, the event will be triggered on all listeners.
  #[serde(rename_all = "camelCase")]
  Emit {
    event: EventId,
    window_label: Option<WindowLabel>,
    payload: Option<JsonValue>,
  },
}

impl Cmd {
  fn listen<R: Runtime>(
    context: InvokeContext<R>,
    event: EventId,
    window_label: Option<WindowLabel>,
    handler: CallbackFn,
  ) -> super::Result<u64> {
    let event_id = rand::random();

    let window_label = window_label.map(|l| l.0);

    context
      .window
      .eval(&listen_js(
        context.window.manager().event_listeners_object_name(),
        format!("'{}'", event.0),
        event_id,
        window_label.clone(),
        format!("window['_{}']", handler.0),
      ))
      .map_err(crate::error::into_anyhow)?;

    context
      .window
      .register_js_listener(window_label, event.0, event_id);

    Ok(event_id)
  }

  fn unlisten<R: Runtime>(
    context: InvokeContext<R>,
    event: EventId,
    event_id: u64,
  ) -> super::Result<()> {
    context
      .window
      .eval(&unlisten_js(
        context.window.manager().event_listeners_object_name(),
        event.0,
        event_id,
      ))
      .map_err(crate::error::into_anyhow)?;
    context.window.unregister_js_listener(event_id);
    Ok(())
  }

  fn emit<R: Runtime>(
    context: InvokeContext<R>,
    event: EventId,
    window_label: Option<WindowLabel>,
    payload: Option<JsonValue>,
  ) -> super::Result<()> {
    // dispatch the event to Rust listeners
    context.window.trigger(
      &event.0,
      // TODO: dispatch any serializable value instead of a string in v2
      payload.as_ref().and_then(|p| {
        serde_json::to_string(&p)
          .map_err(|e| {
            #[cfg(debug_assertions)]
            eprintln!("{}", e);
            e
          })
          .ok()
      }),
    );

    if let Some(target) = window_label {
      context
        .window
        .emit_to(&target.0, &event.0, payload)
        .map_err(crate::error::into_anyhow)?;
    } else {
      context
        .window
        .emit_all(&event.0, payload)
        .map_err(crate::error::into_anyhow)?;
    }
    Ok(())
  }
}
