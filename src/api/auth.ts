import { invoke } from "@tauri-apps/api/tauri";

interface DeviceAuth {
    device_code: string;
    expires_in: number;
    interval: number;
    message: string;
    user_code: string;
    verification_uri: string;
}

export default class Auth {

    private deviceAuth: DeviceAuth | undefined;

    public async getDeviceCode(): Promise<string> {
        const device_auth = await invoke("plugin:auth|device_auth") as DeviceAuth;
        this.deviceAuth = device_auth;
        return Promise.resolve(device_auth.user_code);
    }

    public async auth_verification_await(): Promise<null> {
        return await invoke("plugin:auth|auth_verification_await", { deviceAuth: this.deviceAuth });
    }

}