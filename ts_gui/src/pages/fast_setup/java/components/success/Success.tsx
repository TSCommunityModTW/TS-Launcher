import { useTranslation } from "node_modules/react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Success.module.scss";

import Trail from "@/pages/components/trail/Trail";
import successImg from "@/assets/icons/success.svg";

export default function Success() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.installSuccessContainer}>

            <Trail open={true}>

                <div className={styles.titleContainer}>
                    <img className={styles.successImg} src={successImg} />
                    <h1>{t("java.install_success.text_1")}</h1>
                </div>

                <h2>{t("java.install_success.text_2")}</h2>
                <h2>{t("java.install_success.text_3")}</h2>
                <h2>{t("java.install_success.text_4")}</h2>

                <div className={styles.buttonContainer}>

                    <button
                        onClick={() => {
                            navigate("/setup_success");
                        }}
                    >{t("java.install_success.text_5")}</button>

                </div>

            </Trail>

        </div>
    )
}