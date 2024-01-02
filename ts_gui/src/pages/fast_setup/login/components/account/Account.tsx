import { useTranslation } from "node_modules/react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Account.module.scss";

import Trail from "@/pages/components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

export default function Account() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.accountContainer}>

            <Trail open={true}>

                <h1>{t("login.account.text_1")}</h1>
                <h2>{t("login.account.text_2")}</h2>

                <ButtonFocus
                    content={t("login.account.text_3")}
                    themeColor="purple"
                    onClick={() => {
                        navigate("/login/device_code");
                    }}
                />
                <ButtonFocus
                    className={styles.secondButton}
                    content={t("login.account.text_4")}
                    themeColor="gray"
                    onClick={() => {
                        navigate("/java/setup");
                    }}
                />

            </Trail>

        </div>
    )
}