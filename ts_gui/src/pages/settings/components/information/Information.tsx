import { useTranslation } from "node_modules/react-i18next";
import { open } from "@tauri-apps/api/shell";
import { v4 as uuidv4 } from "uuid";

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
            description: "全端工程師 (程式碼撰寫)",
            img: developerQuasiImg,
            discordLink: "",
            githubLink: "https://github.com/yucheng918"
        },
        {
            name: "小雷",
            description: "UI 架構師 (想法 & 創意構思)",
            img: developerTsImg,
            discordLink: "",
            githubLink: "https://github.com/Raw-air"
        },
        {
            name: "阿倫",
            description: "UI/UX 設計師 (設計 & 繪圖)",
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
                            <div key={uuidv4()} className={styles.developersDiv}>

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