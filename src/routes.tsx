import { createBrowserRouter } from "react-router-dom";

import App from "@/App";
import Login from "@/pages/fast_setup/login/Login";
import Frame from "@/pages/components/frame/Frame";
import Main from "@/pages/main/Main";
import Settings from "@/pages/settings/Settings";
import InstanceSettings from "@/pages/instanceSettings/InstanceSettings";
import Welcome from "@/pages/welcome/Welcome";
import DeviceCode from "@/pages/fast_setup/login/components/deviceCode/DeviceCode";
import Account from "@/pages/fast_setup/login/components/account/Account";
import LinkError from "@/pages/fast_setup/login/components/linkError/LinkError";
import LinkSuccess from "@/pages/fast_setup/login/components/linkSuccess/LinkSuccess";
import Java from "@/pages/fast_setup/java/Java";
import Setup from "@/pages/fast_setup/java/components/setup/Setup";
import Install from "@/pages/fast_setup/java/components/install/Install";
import SetupSuccess from "@/pages/fast_setup/success/Success";
import Paths from "@/pages/fast_setup/java/components/paths/Paths";
import Success from "@/pages/fast_setup/java/components/success/Success";
import CrashPayback from "./pages/components/crashPayback/CrashPayback";

export default createBrowserRouter([
    {
        element: (
            <>
                <Frame windowName="main" osType={"macos"} />
                <CrashPayback />
            </>
        ),
        children: [
            {
                path: "/",
                element: <App />
            },
            {
                path: "/welcome",
                element: <Welcome />,
            },
            {
                path: "/login",
                element: <Login />,
                children: [
                    {
                        path: "/login/account",
                        element: <Account />
                    },
                    {
                        path: "/login/device_code",
                        element: <DeviceCode />
                    },
                    {
                        path: "/login/link_error",
                        element: <LinkError />
                    },
                    {
                        path: "/login/link_success",
                        element: <LinkSuccess />
                    }
                ]
            },
            {
                path: "/java",
                element: <Java />,
                children: [
                    {
                        path: "/java/setup",
                        element: <Setup />
                    },
                    {
                        path: "/java/install",
                        element: <Install />
                    },
                    {
                        path: "/java/install_success",
                        element: <Success />
                    },
                    {
                        path: "/java/paths",
                        element: <Paths />
                    }
                ]
            },
            {
                path: "setup_success",
                element: <SetupSuccess />
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