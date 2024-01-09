import { LoaderFunctionArgs } from "react-router-dom";

import Store from "./invoke/store";
import System from "./invoke/system";
import Launcher from "./invoke/launcher";
import { IStoreSettingsJava } from "./interfaces/IStoreSettingsJava";
import { ILauncherAssetsServer } from "./interfaces/ILauncherAssetsServer";

export interface IParametersLoader {
    storeSettingsJava: IStoreSettingsJava,
    systemMaxMemorySize: number
}

export async function parametersLoader({ params }: LoaderFunctionArgs<any>): Promise<IParametersLoader> {

    let instanceId = params.instanceId;

    if (!instanceId) {
        instanceId = "global";
    }

    const storeSettingsJava = await Store.getSettingsJava(instanceId);
    const systemMaxMemorySize = await System.getMaxMemorySize();

    return { storeSettingsJava, systemMaxMemorySize }
}

export interface IMainLoader {
    servers: Array<ILauncherAssetsServer>
}

export async function mainLoader(): Promise<IMainLoader> {

    const servers = await Launcher.get_assets_servers();
    
    return { servers };
}

export async function serverInfoLoader({ params }: LoaderFunctionArgs<any>): Promise<ILauncherAssetsServer> {

    let serverId = params.serverId;

    const server = await Launcher.get_assets_server(serverId!);

    return server;
}