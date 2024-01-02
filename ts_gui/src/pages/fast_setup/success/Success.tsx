import { useNavigate } from "react-router-dom";
import { useTranslation } from "node_modules/react-i18next";

import styles from "./Success.module.scss";

import logo_3 from "@/assets/images/logo/ts_3.png";
import Trail from "../../components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

export default function SetupSuccess() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.setupSuccessContainer}>

            <div className={styles.leftContainer}>
                <img className={styles.logoImg} src={logo_3} />
            </div>

            <div className={styles.rightContainer}>
                <div className={styles.textButtonContainer}>

                    <Trail open={true}>

                        <h1>{t("setup_success.text_1")}</h1>
                        <h2>{t("setup_success.text_2")}</h2>

                        <div className={styles.buttonContainer}>

                            <ButtonFocus
                                content={t("setup_success.text_3")}
                                themeColor="green"
                                onClick={() => {
                                    navigate("/main");
                                }}
                            />

                            <ButtonFocus
                                content={t("setup_success.text_4")}
                                themeColor="gray"
                                onClick={() => {
                                    
                                }}
                            />

                        </div>

                    </Trail>

                </div>
            </div>

        </div>
    )
}