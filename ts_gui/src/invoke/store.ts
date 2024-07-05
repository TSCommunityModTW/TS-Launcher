import { IStoreProfiles } from "@/interfaces/IStoreProfiles";
import { IStoreSettings } from "@/interfaces/IStoreSettings";
import { IStoreSettingsJava } from "@/interfaces/IStoreSettingsJava";
import { invoke } from "@tauri-apps/api/tauri";

export enum JavaPathVersion {
    Java8 = 0,
    Java16 = 1,
    Java17 = 2
}

export default class Store {
    
    public static async saveLauncherSettingsFile(): Promise<void> {
        await invoke("plugin:store|save");
    }

    public static async getSettings(): Promise<IStoreSettings> {
        return await invoke("plugin:store|settings_get");
    }
  
    public static async setSettings(value: IStoreSettings) {
    public static async getSettingsSelectedServerStart(): Promise<IStoreSettingSelectedServer> {
        return await logger.invokeWithLogging<IStoreSettingSelectedServer>("plugin:store|get_settings_selected_server_start");
    }

    public static async setSettingsSelectedServerStart(value: IStoreSettingSelectedServer) {
        await logger.invokeWithLogging<void>("plugin:store|set_settings_selected_server_start", { value });
    }

    public static async getSettingsJava(id: string): Promise<IStoreSettingsJava> {
        return await invoke("plugin:store|get_settings_java", { id });
    }

    public static async setSettingsJava(id: string, value: IStoreSettingsJava): Promise<void> {
        await invoke("plugin:store|set_settings_java", { id, value });
    }

    public static async getProfiles(): Promise<IStoreProfiles> {
        return await invoke("plugin:store|profiles_get");
    }
  
    public static async setProfiles(value: IStoreProfiles) {
        await invoke("plugin:store|profiles_set", { value });
    }

    public static async clearProfiles() {
        await invoke("plugin:store|profiles_clear");
    }
}