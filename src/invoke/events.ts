import { listen } from "@tauri-apps/api/event";

export async function loading_listener(callback: Function) {
    await listen("loading", (event) => callback(event.payload));
}