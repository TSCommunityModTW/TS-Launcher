import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Install.module.scss";

import Trail from "@/pages/components/trail/Trail";
import { useEffect, useState } from "react";

export default function Install() {

    const navigate = useNavigate();
    const { t } = useTranslation();
    const [progressBarText, setProgressBarText] = useState<string>("Installing Java 17...");
    const [progressBarPercentage, setProgressBarPercentage] = useState<number>(1);

    useEffect(() => {

        setInterval(() => {

            // ! Test
            setProgressBarPercentage((val) => {
                if (val === 100) {
                    
                    setTimeout(() => {
                        navigate("/java/install_success");
                    }, 2000);

                    return val;
                } else {
                    return val + 1;
                }
            });

        }, 50);

    }, []);

    return (
        <div className={styles.installContainer}>

            <Trail open={true}>

                <h1>{t("java.install.text_1")}</h1>

                <div className={styles.versions}>
                    <ul>
                        <li>{t("java.install.text_2")}</li>
                        <li>{t("java.install.text_3")}</li>
                        <li>{t("java.install.text_4")}</li>
                    </ul>
                </div>

                <div className={styles.progressBarContainer}>

                    <div
                        className={styles.progressBar}
                        style={{ width: `${progressBarPercentage}%`}}
                    ></div>

                    <div className={styles.progressBarText}>
                        <h1>{progressBarText}</h1>
                        <h1>{progressBarPercentage}%</h1>
                    </div>

                </div>

            </Trail>

        </div>
    )
}