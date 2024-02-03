import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import Loading from "@/pages/loading/Loading";
import Launcher from "@/invoke/launcher";
import Auth from "./invoke/auth";
import { useAppDispatch } from "./hooks";
import { setCrashOpen } from "./slices/stateSlice";

export default function App() {
    
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const { t } = useTranslation();

    React.useEffect(() => { init() }, []);

    // ! https://github.com/tauri-apps/tauri/issues/5170
    const setupAppWindow = async () => {
        const appWindow = (await import('@tauri-apps/api/window')).appWindow
        appWindow.show();
    }

    // initialize
    const init = async () => {

        setTimeout(setupAppWindow, 200);

        try {

            // Init Launcher
            await Launcher.initialize();

            // Auth
            if (await Auth.auth_verification_expires_at()) {
                navigate("/main");
            } else {
                navigate("/login/device_code/refresh");
            }

        } catch (err: any) {
            dispatch(setCrashOpen({ state: true, errorMessage: err.message }));
        }
    }

    return <Loading text={t("loading.text_1")} />;
}