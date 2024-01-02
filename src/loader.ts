import { LoaderFunctionArgs } from "react-router-dom";

import Store from "./invoke/store";
import System from "./invoke/system";

export async function parametersLoader({ params }: LoaderFunctionArgs<any>) {

    let instanceId = params.instanceId;

    if (!instanceId) {
        instanceId = "global";
    }

    let storeSettingsJava = await Store.getSettingsJava(instanceId);
    let systemMaxMemorySize = await System.getMaxMemorySize();

    return { storeSettingsJava, systemMaxMemorySize }
}