import { invoke } from "@tauri-apps/api/tauri";

export default class System {

    public static async getMaxMemorySize(): Promise<number> {
        return await invoke("plugin:system|get_max_memory_size");
    }

    public static async getFreeMemorySize(): Promise<number> {
        return await invoke("plugin:system|get_free_memory_size");
    }
}