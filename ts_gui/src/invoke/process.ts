import { invoke } from "@tauri-apps/api/tauri";

export default class Process {

    public static async processMinecraftRun(serverId: string, childrenServerId: string): Promise<void> {
        await invoke("plugin:process|process_minecraft_run", { serverId, childrenServerId });
    }
}