import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./ConfirmLogout.module.scss"
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus"


export default function ConfirmLogout(){
    const navigate = useNavigate();
    const { t } = useTranslation();
    return (
        <div className={styles.confirmLogoutContainer}>
            <h1>確定是否登出?</h1>
            <p>
                <ButtonFocus
                    content={t("main.confirmLogout.button_yes")}
                    themeColor="green"
                    onClick={()=>{
                        //需新增移除帳號資料的功能
                        navigate("/main/home");
                    }}
                />
                <ButtonFocus
                    content={t("main.confirmLogout.button_no")}
                    themeColor="gray"
                    onClick={()=>{
                        navigate("/main/home");
                    }}
                />
            </p>
            
        </div>
    )
}