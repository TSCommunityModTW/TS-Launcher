import React from "react";
import Frame from "@/pages/components/Frame/Frame";

export default function App() {

    const [osType, setOSType] = React.useState<"osx" | "windows" | "linux" | "unknown">("unknown");

    React.useEffect(() => {
        init();
    }, []);

    const init = async () => {
        setOSType("osx");
    }

    return (
        <>
            <Frame windowName="main" osType={osType} />

            {/* <HashRouter>
                <Switch>
                    <Route exact path="/"><Init /></Route>
                    <Route path="/main"><Main /></Route>
                    <Route path="/login"><Login /></Route>
                    <Route path="/settings"><Setting /></Route>
                    <Route path="/instanceSetting/:serverId/:paramsMenuType"><InstanceSetting /></Route>
                </Switch>
            </HashRouter> */}
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