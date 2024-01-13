import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import Loading from "@/pages/loading/Loading";
import Launcher from "@/invoke/launcher";

export default function App() {
    
    const navigate = useNavigate();

    React.useEffect(() => { init() }, []);

    // ! https://github.com/tauri-apps/tauri/issues/5170
    const setupAppWindow = async () => {
        const appWindow = (await import('@tauri-apps/api/window')).appWindow
        appWindow.show();
    }

    // initialize
    const init = async () => {

        setTimeout(setupAppWindow, 200);

        await Launcher.initialize();
        // HI

        // navigate("/instanceSettings/nr-server/parameters");
        // navigate("/main/namelessrealms/server_info")
    }

    const { t } = useTranslation();

    return (
        <>
            <Loading text={t("loading.text_1")} />
        </>
    );
}

// function Init() {

//     const { t } = useTranslation();
//     const history = useHistory();

//     React.useEffect(() => {
//         // validateAccessToken(history);
//     }, []);

//     return (
//         <InitLoading text={t("loading.text_1")} />
//     );
// }

// async function validateAccessToken(history: any) {
//     if(await window.electron.auth.isValidateAccessToken()) {
//         history.push("/main");
//     } else {
//         history.push("/login");
//     }
// }