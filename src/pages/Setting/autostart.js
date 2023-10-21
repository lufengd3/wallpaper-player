import { invoke } from '@tauri-apps/api';

export async function isEnabled() {
  return await invoke('plugin:autostart|is_enabled')
}

export async function enable() {
  await invoke('plugin:autostart|enable')
}

export async function disable() {
  await invoke('plugin:autostart|disable')
}