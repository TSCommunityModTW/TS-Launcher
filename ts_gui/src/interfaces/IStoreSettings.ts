// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { IStoreSettingsGeneral } from "./IStoreSettingsGeneral";
import type { IStoreSettingsJava } from "./IStoreSettingsJava";

export interface IStoreSettings { language: string, java: Record<string, IStoreSettingsJava>, display_position: number, launcher_keep_open: boolean, selected_server_start: string, general: IStoreSettingsGeneral, }