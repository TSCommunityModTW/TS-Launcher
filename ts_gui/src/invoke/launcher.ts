import { ILauncherAssets } from "@/interfaces/ILauncherAssets";
import { ILauncherAssetsServer } from "@/interfaces/ILauncherAssetsServer";
import { ILauncherAssetsServerChildren } from "@/interfaces/ILauncherAssetsServerChildren";
import { invoke } from "@tauri-apps/api/tauri";

export default class Launcher {

    public static async initialize(): Promise<void> {
        await invoke("initialize");
    }

    public static async get_assets_info(): Promise<ILauncherAssets> {
        return await invoke("plugin:launcher|get_assets_info");
    }

    public static async get_assets_servers(): Promise<Array<ILauncherAssetsServer>> {
        return await await invoke("plugin:launcher|get_assets_servers");
    }

    public static async get_assets_server(id: string): Promise<ILauncherAssetsServer> {
        return await await invoke("plugin:launcher|get_assets_server", { id });
    }

    public static async get_assets_children_server(mainServerId: string, id: string): Promise<ILauncherAssetsServerChildren> {
        return await await invoke("plugin:launcher|get_assets_children_server", { mainServerId, id });
    }
}   