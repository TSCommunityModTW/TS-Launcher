import { createBrowserRouter } from "react-router-dom";

import App from "@/App";
import Login from "@/pages/login/Login";
import Frame from "@/pages/components/frame/Frame";
import Main from "@/pages/main/Main";
import Settings from "@/pages/settings/Settings";
import InstanceSettings from "@/pages/instanceSettings/InstanceSettings";

export default createBrowserRouter([
    {
        element: <Frame windowName="main" osType={"macos"} />,
        children: [
            {
                path: "/",
                element: <App />
            },
            {
                path: "/login",
                element: <Login />
            },
            {
                path: "/main",
                element: <Main />
            },
            {
                path: "/settings",
                element: <Settings />,
            },
            {
                path: "/instanceSettings",
                element: <InstanceSettings />
            }
        ]
    }
]);