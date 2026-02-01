//! WASM linker and host function registration

use anyhow::Result;
use wasmtime::{Caller, Engine, Linker};

use super::state::HostState;
use crate::plugins::api::NotificationLevel;

/// Create a linker with host functions
pub(super) fn create_linker(engine: &Engine) -> Result<Linker<HostState>> {
    let mut linker = Linker::new(engine);

    // Register host functions for UI API
    linker.func_wrap(
        "claude_visual",
        "show_notification",
        |mut caller: Caller<'_, HostState>, title_ptr: i32, title_len: i32, level: i32| {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory());
            if let Some(memory) = memory {
                let data = memory.data(&caller);
                let title_start = title_ptr as usize;
                let title_end = title_start + title_len as usize;

                if title_end <= data.len() {
                    let title = String::from_utf8_lossy(&data[title_start..title_end]).to_string();
                    let ext_id = caller.data().extension_id.clone();
                    let api = caller.data().api.clone();

                    let level = match level {
                        0 => NotificationLevel::Info,
                        1 => NotificationLevel::Success,
                        2 => NotificationLevel::Warning,
                        _ => NotificationLevel::Error,
                    };

                    api.read().ui.show_notification(&ext_id, &title, None, level);
                    return 0i32;
                }
            }
            -1i32
        },
    )?;

    // Register host function for logging
    linker.func_wrap(
        "claude_visual",
        "log",
        |mut caller: Caller<'_, HostState>, msg_ptr: i32, msg_len: i32, level: i32| {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory());
            if let Some(memory) = memory {
                let data = memory.data(&caller);
                let msg_start = msg_ptr as usize;
                let msg_end = msg_start + msg_len as usize;

                if msg_end <= data.len() {
                    let message = String::from_utf8_lossy(&data[msg_start..msg_end]).to_string();
                    let ext_id = &caller.data().extension_id;

                    match level {
                        0 => tracing::debug!("[{}] {}", ext_id, message),
                        1 => tracing::info!("[{}] {}", ext_id, message),
                        2 => tracing::warn!("[{}] {}", ext_id, message),
                        _ => tracing::error!("[{}] {}", ext_id, message),
                    }
                }
            }
        },
    )?;

    // Register host function for getting API version
    linker.func_wrap(
        "claude_visual",
        "api_version",
        |_caller: Caller<'_, HostState>| -> (i32, i32, i32) {
            // Return version 0.1.0
            (0, 1, 0)
        },
    )?;

    // Register host function for settings get
    linker.func_wrap(
        "claude_visual",
        "settings_get",
        |mut caller: Caller<'_, HostState>, key_ptr: i32, key_len: i32| -> i32 {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory());
            if let Some(memory) = memory {
                let data = memory.data(&caller);
                let key_start = key_ptr as usize;
                let key_end = key_start + key_len as usize;

                if key_end <= data.len() {
                    let key = String::from_utf8_lossy(&data[key_start..key_end]).to_string();
                    let ext_id = caller.data().extension_id.clone();
                    let api = caller.data().api.clone();

                    let result = api.read().settings.get(&ext_id, &key);
                    if result.is_ok() {
                        return 0;
                    }
                }
            }
            -1
        },
    )?;

    // Register host function for settings set
    linker.func_wrap(
        "claude_visual",
        "settings_set",
        |mut caller: Caller<'_, HostState>,
         key_ptr: i32, key_len: i32,
         value_ptr: i32, value_len: i32| -> i32 {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory());
            if let Some(memory) = memory {
                let data = memory.data(&caller);
                let key_start = key_ptr as usize;
                let key_end = key_start + key_len as usize;
                let value_start = value_ptr as usize;
                let value_end = value_start + value_len as usize;

                if key_end <= data.len() && value_end <= data.len() {
                    let key = String::from_utf8_lossy(&data[key_start..key_end]).to_string();
                    let value = String::from_utf8_lossy(&data[value_start..value_end]).to_string();
                    let ext_id = caller.data().extension_id.clone();
                    let api = caller.data().api.clone();

                    api.read().settings.set(&ext_id, &key, &value);
                    return 0;
                }
            }
            -1
        },
    )?;

    Ok(linker)
}
