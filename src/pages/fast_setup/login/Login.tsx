import { Outlet } from "react-router-dom";

import styles from "./Login.module.scss";

import microsoftImg from "@/assets/images/logo/microsoft.svg";
import minecraftImg from "@/assets/images/logo/minecraft_1.svg";

export default function Login() {
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