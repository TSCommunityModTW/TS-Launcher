import React from "react";
import { useTranslation } from "node_modules/react-i18next";

import styles from "./Screenshot.module.scss";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import ImageTool from "@/pages/components/imageTool/ImageTool";

export default function Screenshot() {

    const { t } = useTranslation();
    const [screenshots, _setScreenshots] = React.useState<Array<{ fileName: string; filePath: string; imageSrc: string | undefined }>>();

    React.useEffect(() => {
        // const screenshots = window.electron.game.screenshot.getScreenshots(props.serverId);
        // if (screenshots.length > 0) setScreenshots(screenshots);
    }, []);

    return (
        <div className={styles.screenshotDiv}>

            <h1 className={styles.headline}>{t("instanceSetting.menu.title_2.subTitle_1")}</h1>

            <div className={styles.topDiv}>
                <ButtonFocus content={t("instanceSetting.components.screenshot.topTools.button_1.title") as string} className={styles.buttonFocus} onClick={() => {
                    // const screenshotsDirPath = window.electron.game.screenshot.getScreenshotsDirPath(props.serverId);
                    // window.electron.open.pathFolder(screenshotsDirPath);
                }} />
            </div>

            <div className={styles.listDiv}>
                {
                    screenshots !== undefined
                        ?
                        screenshots.map((item) => (
                            <ImageTool type="Screenshots" title={item.fileName} filePath={item.filePath} imageSrc={item.imageSrc} onDeleteClick={(_filePath) => {
                                // window.electron.game.screenshot.screenshotDelete(filePath);
                                // setScreenshots(window.electron.game.screenshot.getScreenshots(props.serverId));
                            }} />
                        ))
                        :
                        <div className={styles.notScreenshots}>
                            <h1>{t("instanceSetting.components.screenshot.notScreenshots.title")}</h1>
                        </div>
                }
            </div>

        </div>
    );
}