import { invoke } from '@tauri-apps/api/tauri';

// Function to log messages
export default class logger {
    public static async logMessage(level: string, message: string) {
        await invoke("plugin:logger|log_message", { level, message })
            .then(() => {
                console.log(`Log [${level}] sent to Rust backend successfully.`);
            })
            .catch((error) => {
                console.error(`Failed to send log [${level}] to Rust backend:`, error);
            });
    }

    public static async invokeWithLogging<T>(command: string, args?: Record<string, unknown>): Promise<T> {
        try {
            logger.logMessage("debug", `Invoking command: ${command} with args: ${JSON.stringify(args)}`);
            const result = await invoke<T>(command, args);
            logger.logMessage("info", `Command ${command} executed successfully.`);
            return result;
        } catch (error) {
            if (error instanceof Error) {
                logger.logMessage("error", `Error in command ${command}: ${error.message}`);
            } else {
                logger.logMessage("error", `Error in command ${command}: ${JSON.stringify(error)} `);
            }
            throw error; 
        }
    }
}