import { invoke } from "@tauri-apps/api/tauri";

export default class Java {

    public static async autoInstallJava(): Promise<void> {
        await invoke("plugin:java|auto_install_all_java");
    }

    public static async test_jar(path: string): Promise<boolean> {
        return await invoke("plugin:java|test_jar", { path });
    }
}