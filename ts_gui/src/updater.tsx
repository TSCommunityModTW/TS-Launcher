//These code are copy from https://tauri.app/v1/api/js/updater/

import { checkUpdate, installUpdate, onUpdaterEvent } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

import logger from './invoke/logger'


export default async function updater() {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
        // This will log all updater events, including status updates and errors.
        logger.logMessage("error", `updater.tsx: Error message:${error},Status:${status}`)
    })

    try {
        const { shouldUpdate, manifest } = await checkUpdate()


        if (shouldUpdate) {
            // You could show a dialog asking the user if they want to install the update here.
            logger.logMessage("info", `updater.tsx: Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}` )

            // Install the update. This will also restart the app on Windows!
            await installUpdate()

            // On macOS and Linux you will need to restart the app manually.
            // You could use this step to display another confirmation dialog.
            await relaunch()
        }
    } catch (error) {
        logger.logMessage("error",`Updater ${error}`)
        console.error(error)
    }

    // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
    unlisten()
}
