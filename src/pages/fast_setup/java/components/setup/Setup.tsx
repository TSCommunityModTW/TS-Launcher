import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Setup.module.scss";

import Trail from "@/pages/components/trail/Trail";

export default function Setup() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.setupContainer}>

            <Trail open={true}>

                <h1>{t("java.setup.text_1")}</h1>

                <div className={styles.versions}>
                    <ul>
                        <li>{t("java.setup.text_2")}</li>
                        <li>{t("java.setup.text_3")}</li>
                        <li>{t("java.setup.text_4")}</li>
                    </ul>
                </div>

                <h2>{t("java.setup.text_5")}</h2>
                <h2>{t("java.setup.text_6")}</h2>
                <h2>{t("java.setup.text_7")}</h2>

                <div className={styles.buttonContainer}>

                    <button
                        onClick={() => {
                            navigate("/java/install");
                        }}
                    >{t("java.setup.text_8")}</button>

                    <button className={styles.secondButton}
                        onClick={() => {
                            navigate("/java/paths");
                        }}
                    >{t("java.setup.text_9")}</button>

                </div>

            </Trail>

        </div>
    )
}