import { IMicrosoftDeviceAuth } from "@/interfaces/IMicrosoftDeviceAuth";
import { invoke } from "@tauri-apps/api/tauri";

export default class Auth {

    private deviceAuth: IMicrosoftDeviceAuth | undefined;

    public async getDeviceCode(): Promise<string> {
        const device_auth = await invoke("plugin:auth|get_device_code") as IMicrosoftDeviceAuth;
        this.deviceAuth = device_auth;
        return Promise.resolve(device_auth.user_code);
    }

    public async auth_minecraft_await(): Promise<boolean> {
        return await invoke("plugin:auth|auth_minecraft_await", { deviceAuth: this.deviceAuth });
    }

}