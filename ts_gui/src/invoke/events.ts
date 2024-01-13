import { IEventLoadingPayload } from "@/interfaces/IEventLoadingPayload";
import { listen } from "@tauri-apps/api/event";

export async function loading_listener(callback: (payload: IEventLoadingPayload) => void) {
    await listen("loading", (event: any) => callback(event.payload));
}