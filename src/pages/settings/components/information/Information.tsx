import { useTranslation } from "react-i18next";
import { open } from "@tauri-apps/api/shell";

import styles from "./Information.module.scss";

import developerQuasiImg from "@/assets/images/personnel/yu.jpg";
import developerTsImg from "@/assets/images/personnel/ts.png";
import developerGxsImg from "@/assets/images/personnel/gxs.png";
import githubIconImg from "@/assets/icons/github.png";
import discordIconImg from "@/assets/icons/discord.svg";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

export default function Information() {

    const { t } = useTranslation();

    const developers = [
        {
            name: "Yu",
            description: "主要代碼撰寫",
            img: developerQuasiImg,
            discordLink: "",
            githubLink: "https://github.com/yucheng918"
        },
        {
            name: "小雷",
            description: "UI設計師",
            img: developerTsImg,
            discordLink: "",
            githubLink: "https://github.com/Raw-air"
        },
        {
            name: "阿倫",
            description: "前端工程師",
            img: developerGxsImg,
            discordLink: "",
            githubLink: "https://github.com/kocreeper1"
        }
    ]

    return (
        <div className={styles.informationDiv}>

            <div className={styles.launcherInfoDiv}>

                <div className={styles.leftDiv}>
                    <h1>{t("setting.components.information.launcherInfo.title_1")}</h1>
                    <h2>{t("setting.components.information.launcherInfo.title_2")} 0.1.0-BETA</h2>
                </div>
                {/* <div className={styles.rightDiv}>
                    
                </div> */}

            </div>

            <div className={styles.developersListDiv}>

                {
                    developers.map((developer) => {
                        return (
                            <div className={styles.developersDiv}>

                                <div className={styles.leftDiv}>

                                    <div className={styles.leftLeftDiv}>
                                        <img src={developer.img} />
                                    </div>

                                    <div className={styles.leftRightDiv}>
                                        <h1>{developer.name}</h1>
                                        <h2>{developer.description}</h2>
                                    </div>

                                </div>

                                <div className={styles.rightDiv}>
                                    <img src={discordIconImg} onClick={() => open(developer.discordLink)} />
                                    <img src={githubIconImg} onClick={() => open(developer.githubLink)} />
                                </div>

                            </div>
                        )
                    })
                }

            </div>

            <ButtonFocus className={styles.buttonFocus} content="回報啟動器錯誤" onClick={() => window.open("https://github.com/TSCommunityModTW/TS-Launcher/issues/new/choose")} />

        </div>
    );
}