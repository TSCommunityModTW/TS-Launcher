import { useTranslation } from "node_modules/react-i18next";

import styles from "./LinkError.module.scss";

import Trail from "@/pages/components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import { useNavigate } from "react-router-dom";

export default function LinkError() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.linkErrorContainer}>

            <Trail open={true}>

                <h1>{t("login.link_error.text_1")}</h1>
                <h2>{t("login.link_error.text_2")}</h2>

                <div className={styles.buttonContainer}>

                    <ButtonFocus
                        content={t("login.link_error.text_3")}
                        themeColor="purple"
                        onClick={() => {
                            navigate("/login/device_code");
                        }}
                    />
                    <ButtonFocus
                        content={t("login.link_error.text_4")}
                        themeColor="gray"
                        onClick={() => {
                            navigate("/java/setup");
                        }}
                    />

                </div>

            </Trail>

        </div>
    )
}