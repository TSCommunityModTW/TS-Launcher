import { IStoreProfiles } from "@/interfaces/IStoreProfiles";
import { IStoreSettingSelectedServer } from "@/interfaces/IStoreSettingSelectedServer";
import { IStoreSettings } from "@/interfaces/IStoreSettings";
import { IStoreSettingsJava } from "@/interfaces/IStoreSettingsJava";


import logger from "./logger";

export enum JavaPathVersion {
    Java8 = 0,
    Java16 = 1,
    Java17 = 2
}

export default class Store {

    public static async saveLauncherSettingsFile(): Promise<void> {
        await logger.invokeWithLogging<void>("plugin:store|save");
    }

    public static async getSettings(): Promise<IStoreSettings> {
        return await logger.invokeWithLogging<IStoreSettings>("plugin:store|settings_get");
    }

    public static async setSettings(value: IStoreSettings) {
        await logger.invokeWithLogging<void>("plugin:store|settings_set", { value });
    }

    public static async getSettingsSelectedServerStart(): Promise<IStoreSettingSelectedServer> {
        return await logger.invokeWithLogging<IStoreSettingSelectedServer>("plugin:store|get_settings_selected_server_start");
    }

    public static async setSettingsSelectedServerStart(value: IStoreSettingSelectedServer) {
        await logger.invokeWithLogging<void>("plugin:store|set_settings_selected_server_start", { value });
    }

    public static async getSettingsJava(id: string): Promise<IStoreSettingsJava> {
        return await logger.invokeWithLogging<IStoreSettingsJava>("plugin:store|get_settings_java", { id });
    }

    public static async setSettingsJava(id: string, value: IStoreSettingsJava): Promise<void> {
        await logger.invokeWithLogging<void>("plugin:store|set_settings_java", { id, value });
    }

    public static async getProfiles(): Promise<IStoreProfiles> {
        return await logger.invokeWithLogging<IStoreProfiles>("plugin:store|profiles_get");
    }

    public static async setProfiles(value: IStoreProfiles) {
        await logger.invokeWithLogging<void>("plugin:store|profiles_set", { value });
    }

    public static async clearProfiles() {
        await logger.invokeWithLogging<void>("plugin:store|profiles_clear");
    }
}