import { useTranslation } from "react-i18next";
import { Outlet, useNavigate } from "react-router-dom";

import styles from "./Login.module.scss";

import microsoftImg from "@/assets/images/logo/microsoft.svg";
import minecraftImg from "@/assets/images/logo/minecraft_1.svg";

export default function Login() {

    const { t } = useTranslation();
    const navigate = useNavigate();

    return (
        <div className={styles.loginContainer}>

            <div className={styles.leftContainer}>
                <img className={styles.microsoftImg} src={microsoftImg} />
                <img className={styles.minecraftImg} src={minecraftImg} />
            </div>

            <div className={styles.rightContainer}>
                <Outlet />
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