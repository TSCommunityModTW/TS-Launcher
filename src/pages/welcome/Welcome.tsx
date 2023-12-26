import { useTranslation } from "react-i18next";
import Trail from "../components/trail/Trail";

import styles from "./Welcome.module.scss";

import logo_2 from "@/assets/images/logo/ts_2.png";
import { useNavigate } from "react-router-dom";
import ButtonFocus from "../components/buttonFocus/ButtonFocus";

export default function Welcome() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.welcomeContainer}>

            <div className={styles.leftContainer}>
                <img className={styles.logoImg} src={logo_2} />
            </div>

            <div className={styles.rightContainer}>
                <div className={styles.textButtonContainer}>

                    <Trail open={true}>

                        <h1>{t("welcome.text_1")}</h1>
                        <h2>{t("welcome.text_2")}</h2>

                        <ButtonFocus
                            content={t("welcome.text_3")}
                            themeColor="purple"
                            className={styles.button}
                            onClick={() => {
                                navigate("/login/account")
                            }}
                        />

                        <ButtonFocus
                            content={t("welcome.text_4")}
                            themeColor="gray"
                            className={`${styles.button} ${styles.secondButton}`}
                            onClick={() => {
                                navigate("/main");
                            }}
                        />

                    </Trail>

                </div>
            </div>

        </div>
    )
}