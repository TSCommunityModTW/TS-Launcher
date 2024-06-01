import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./ConfirmLogout.module.scss"
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus"
import Store from "@/invoke/store";
import { IStoreProfiles } from "@/interfaces/IStoreProfiles";


export default function ConfirmLogout(){
    const navigate = useNavigate();
    const { t } = useTranslation();
    const emptyProfile: IStoreProfiles = {
        microsoft_auth: {
            mc_account_token: "",
            expires_at: "0",
        },
        user: {
            username: "",
            id: "",
        },
        player: {
            name: "",
            uuid: "",
        },
    };
    return (
        <div className={styles.confirmLogoutContainer}>
            <h1>確定是否登出?</h1>
            <p>
                <ButtonFocus
                    content={t("main.confirmLogout.button_yes")}
                    themeColor="green"
                    onClick={async()=>{
                        await Store.setProfiles(emptyProfile);
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