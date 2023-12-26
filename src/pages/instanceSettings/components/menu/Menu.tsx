import React from "react";
import { useTranslation } from "react-i18next";

import styles from "./Menu.module.scss";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

type IProps = {
    menuType: number;
    serverId: string;
    onClickMenuButton?: (menuType: number) => void;
}

export default function menu(props: IProps) {

    const { t } = useTranslation();
    const [menuType, setMenuType] = React.useState(props.menuType);

    const onClickMenuButton = (type: number) => {

        setMenuType(type);

        if (props.onClickMenuButton !== undefined) {
            props.onClickMenuButton(type);
        }
    }

    const componentStyles = (index: number) => {

        const styles = [];

        if (menuType === index) {
            styles.push({ background: "#A238FF", color: "#ffff" });
        }

        return styles.reduce((acc, style) => ({ ...acc, ...style }), {});
    }

    return (
        <div className={styles.menuDiv}>

            <div className={styles.menuContentDiv}>

                <div className={styles.menuContentButtonDiv}>

                    <h1>{t("instanceSetting.menu.title_1.title")}</h1>
                    <div>
                        <h2
                            style={componentStyles(1)}
                            onClick={() => onClickMenuButton(1)}
                        >{t("instanceSetting.menu.title_1.subTitle_1")}</h2>
                        <h2
                            style={componentStyles(2)}
                            onClick={() => onClickMenuButton(2)}
                        >{t("instanceSetting.menu.title_1.subTitle_2")}</h2>
                        <h2
                            style={componentStyles(3)}
                            onClick={() => onClickMenuButton(3)}
                        >{t("instanceSetting.menu.title_1.subTitle_3")}</h2>
                    </div>

                    <div className={styles.tr}></div>

                    <h1>{t("instanceSetting.menu.title_2.title")}</h1>
                    <div>
                        <h2
                            style={componentStyles(4)}
                            onClick={() => onClickMenuButton(4)}
                        >{t("instanceSetting.menu.title_2.subTitle_1")}</h2>
                    </div>

                    {/* <div className={styles.tr}></div> */}

                    {/* <h1>{t("instanceSetting.menu.title_3.title")}</h1>
                    <div>
                        <h2
                            style={menuType === 5 ? { background: "#1E1E1E", color: "#ffff" } : {}}
                            onClick={() => onClickMenuButton(5)}
                        >{t("instanceSetting.menu.title_3.subTitle_1")}</h2>
                    </div> */}

                    {/* <div className={styles.functionButtonDiv}>
                        <ButtonFocus className={`${styles.functionButton} ${styles.functionFixButton}`} content="掃描與修復" />
                    </div> */}

                    <div className={styles.tr}></div>

                    <h1>{t("instanceSetting.menu.title_4.title")}</h1>
                    <div className={styles.functionButtonDiv}>
                        <ButtonFocus className={styles.functionButton} content={t("instanceSetting.menu.title_4.button_1") as string} onClick={() => {
                            // const gameMinecraftDirPath = window.electron.path.getGameMinecraftDirPath(props.serverId);
                            // window.electron.open.pathFolder(gameMinecraftDirPath);
                        }}/>
                        <ButtonFocus className={styles.functionButton} content={t("instanceSetting.menu.title_4.button_2") as string} onClick={() => {
                            // const gameModsDirPath = window.electron.path.getGameModsDirPath(props.serverId);
                            // window.electron.open.pathFolder(gameModsDirPath);
                        }} />
                    </div>

                </div>

            </div>

        </div>
    );
}