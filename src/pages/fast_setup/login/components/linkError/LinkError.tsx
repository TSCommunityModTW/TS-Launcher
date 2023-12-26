import { useTranslation } from "react-i18next";

import styles from "./LinkError.module.scss";

import Trail from "@/pages/components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

export default function LinkError() {

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

                        }}
                    />
                    <ButtonFocus
                        content={t("login.link_error.text_4")}
                        themeColor="gray"
                        onClick={() => {
                            
                        }}
                    />

                </div>

            </Trail>

        </div>
    )
}