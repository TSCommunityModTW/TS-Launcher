import { v4 as uuidv4 } from "uuid";
import { useTranslation } from "react-i18next";

import styles from "./Changelog.module.scss";

enum ChangelogContextColor {
    Green = "#0dc468",
    Red = "#e93232",
    Yellow = "#dda50c"
}

interface ChangelogContext {
    title: string,
    color: ChangelogContextColor,
    descriptions: Array<string>
}

export default function Changelog() {

    const { t } = useTranslation();

    const contextList: Array<ChangelogContext> = [
        {
            title: "v0.1.1",
            color: ChangelogContextColor.Green,
            descriptions: [
                "- 少量錯誤修復"
            ]
        },
        {
            title: "新專案開發",
            color: ChangelogContextColor.Green,
            descriptions: [
                "新增：啟動器 v0.1.0-beta 專案"
            ]
        },
    ]

    return (
        <div className={styles.changelogDiv}>

            <div className={styles.changelogTitleDiv}>
                <h1 className={styles.headline}>{t("setting.menu.title_2.subTitle_1")}</h1>
                <h2 className={styles.versionText}>{`v0.1.1-beta`}</h2>
            </div>

            {
                contextList.map(item => (
                    <div key={uuidv4()} className={styles.item}>

                        <div className={styles.titleDiv}>
                            <h1 className={styles.title} style={{ color: item.color }}>{item.title}</h1>
                            {/* <div className={styles.titleTr}></div> */}
                        </div>

                        {
                            item.descriptions.map(item => (
                                <div key={uuidv4()} className={styles.descriptionItem}>
                                    <div className={styles.circle}></div>
                                    <h1 className={styles.descriptionText}>{item}</h1>
                                </div>
                            ))
                        }

                    </div>
                ))
            }

        </div>
    );
}