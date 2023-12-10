import { useTranslation } from "react-i18next";

import styles from "./Information.module.scss";

import developerQuasiImg from "@/assets/images/developers/Yu.jpg";
import githubIconImg from "@/assets/icons/github.png";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

export default function Information() {

    const { t } = useTranslation();

    return (
        <div className={styles.informationDiv}>
            
            <div className={styles.launcherInfoDiv}>

                <div className={styles.leftDiv}>
                    <h1>{t("setting.components.information.launcherInfo.title_1")}</h1>
                    <h2>{t("setting.components.information.launcherInfo.title_2")} v0.1.0</h2>
                </div>
                <div className={styles.rightDiv}>
                    <ButtonFocus className={styles.buttonFocus} content="回報啟動器錯誤" onClick={() => window.open("https://github.com/TSCommunityModTW/TS-Launcher/issues/new/choose")} />
                </div>

            </div>

            <div className={styles.developersListDiv}>

                <div className={styles.developersDiv}>

                    <div className={styles.leftDiv}>

                        <div className={styles.leftLeftDiv}>
                            <img src={developerQuasiImg} alt="Quasi" />
                        </div>

                        <div className={styles.leftRightDiv}>
                            <h1>Yu</h1>
                            <h2>{t("setting.components.information.developersList.developers_1.description")}</h2>
                        </div>

                    </div>

                    <div className={styles.rightDiv}>
                        <img src={githubIconImg} alt="github-icon" onClick={() => window.open("https://github.com/yucheng918")} />
                    </div>

                </div>

            </div>

        </div>
    );
}