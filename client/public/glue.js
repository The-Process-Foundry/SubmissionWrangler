const invoke = window.__TAURI__.invoke;

export async function call(name) {
  return await invoke("call", { name: name });
}
