import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Login.module.scss";

import logo_2 from "@/assets/images/logo/logo_2.png";
import Trail from "@/pages/components/trail/Trail";

export default function Login() {

    const { t } = useTranslation();
    const navigate = useNavigate();

    return (
        <div className={styles.loginContainer}>

            <div className={styles.loginImgContainer}>
                <img className={styles.loginImg} src={logo_2} />
            </div>

            <div className={styles.loginTextContainer}>
                <div className={styles.loginTextButtonContainer}>

                    <Trail open={true}>
                        
                        <h1>{t("login.title")}</h1>
                        <h2>{t("login.subTitle_1")}</h2>

                        // TODO: login
                        <button>{t("login.button_1_Text")}</button>

                        <button className={styles.secondButton}
                            onClick={() => {
                                navigate("/main");
                            }}
                        >{t("login.button_2_Text")}</button>

                    </Trail>

                </div>
            </div>

        </div>
    );
}

// function microsoftLogin(history: any, setLoading: Function): void {

//     setLoading(true);

//     window.electron.auth.microsoftLogin.openLoginWindow(true, (code) => {
//         switch(code) {
//             case 0:
//                 history.push("/main");
//                 break;
//             case 1:
//             case 2:
//                 setLoading(false);
//                 break;
//         }
//     });
// }

// function minecraftLogin(email: string, password: string, loginKeepToggle: boolean, history: any, setLoading: Function): void {

//     setLoading(true);

//     window.electron.auth.mojangLogin.login(email, password, loginKeepToggle, (code) => {
//         if (code === 0) {
//             history.push("/main");
//         } else {
//             setLoading(false);
//         }
//     });
// }