const invoke = window.__TAURI__.invoke;

export async function call_server(name) {
  console.log("Sending " + name + " to the sever");
  let result = await invoke("call_server", { message: name });
  console.log("Received response: " + result);
  return result;
}
