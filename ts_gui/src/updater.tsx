//These code are copy from https://tauri.app/v1/api/js/updater/

import { checkUpdate, installUpdate, onUpdaterEvent } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

import logger from './invoke/logger'


export default async function updater() {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
        switch (status){
            case "ERROR":{
                logger.logMessage("error", `updater.tsx: Error message:${error}`);
                break
            }
            case "PENDING":{
                logger.logMessage("info", `updater.tsx: Pending to update`);
                break
            }
            case "UPTODATE":{
                logger.logMessage("info", `updater.tsx: This version Up to date`);
                break
            }
            case "DONE":{
                logger.logMessage("info", `updater.tsx: The update finished.`);
                break
            }
        }
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
        logger.logMessage("error",`updater.tsx: Update failed, error: ${error}`)
    }

    // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
    unlisten()
}

export async function isNewUpdate() {
    try {
        const { shouldUpdate, manifest } = await checkUpdate()


        if (shouldUpdate) {
            logger.logMessage("info", `updater.tsx: There are new update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}` )
            return true;
        }
        else{
            logger.logMessage("info", `updater.tsx: There are no new update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}` )
            return false;
        }
    } catch (error) {
        logger.logMessage("error",`updater.tsx: Check update failed. Error: ${error}`)
        return false;
    }
}
