import { invoke } from '@tauri-apps/api/tauri';




// Function to log messages
export default class logger {
    public static async logMessage(level: string, message: string) {
        invoke('log_message', { level, message })
        await invoke("plugin:logger|log_message", { level, message })
            .then(() => {
                console.log(`Log [${level}] sent to Rust backend successfully.`);
            })
            .catch((error) => {
                console.error(`Failed to send log [${level}] to Rust backend:`, error);
            });
    }
}