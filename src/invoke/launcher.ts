import { invoke } from "@tauri-apps/api/tauri";

export default class Launcher {

    public static async initialize(): Promise<void> {
        await invoke("initialize");
    }
}   