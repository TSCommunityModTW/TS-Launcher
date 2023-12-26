import { invoke } from "@tauri-apps/api/tauri";

export default class Player {

    public static async get_name(): Promise<string> {
        return await invoke("plugin:player|get_player_name");
    }
}