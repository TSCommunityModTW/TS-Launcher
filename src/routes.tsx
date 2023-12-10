import { createBrowserRouter } from "react-router-dom";

import App from "@/App";
import Login from "@/pages/login/Login";
import Frame from "@/pages/components/frame/Frame";
import Main from "@/pages/main/Main";
import Setting from "@/pages/setting/Setting";

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
                element: <Setting />
            }
        ]
    }
]);