import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import Loading from "@/pages/loading/Loading";

export default function App() {
    
    const navigate = useNavigate();

    React.useEffect(() => { init() }, []);

    // initialize
    const init = async () => {

        // navigate("/main");
        // setTimeout(() => {
        //     navigate("/login");
        // }, 5000);
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