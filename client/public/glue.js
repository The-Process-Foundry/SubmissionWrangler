const invoke = window.__TAURI__.invoke;

export async function call_server(name) {
  console.log("Sending " + name + " to the sever");
  return await invoke("call_server", { message: name });
}
