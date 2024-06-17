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
import ConfirmLogout from "@/pages/main/pages/confirmLogout/ConfirmLogout";
import Setup from "@/pages/fast_setup/java/components/setup/Setup";
import Install from "@/pages/fast_setup/java/components/install/Install";
import SetupSuccess from "@/pages/fast_setup/success/Success";
import Paths from "@/pages/fast_setup/java/components/paths/Paths";
import Success from "@/pages/fast_setup/java/components/success/Success";
import CrashPayback from "@/pages/components/crashPayback/CrashPayback";
import General from "@/pages/settings/components/general/General";
import Parameters from "@/pages/components/parameters/Parameters";
import Language from "@/pages/settings/components/language/Language";
import Changelog from "@/pages/settings/components/changelog/Changelog";
import Information from "@/pages/settings/components/information/Information";
import ModList from "@/pages/instanceSettings/components/modList/ModList";
import ResourcePacks from "@/pages/instanceSettings/components/resourcePacks/ResourcePacks";
import Screenshot from "@/pages/instanceSettings/components/screenshot/Screenshot";
import ServerInfo from "@/pages/main/pages/serverInfo/ServerInfo";
import Home from "@/pages/main/pages/home/Home";

import { mainLoader, parametersLoader, serverInfoLoader } from "./loader";

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
                        path: "/login/device_code/:type",
                        element: <DeviceCode />
                    },
                    {
                        path: "/login/link_error",
                        element: <LinkError />
                    },
                    {
                        path: "/login/link_success/:type",
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
                element: <Main />,
                loader: mainLoader,
                children: [
                    {
                        path: "/main/:serverId/server_info",
                        element: <ServerInfo />,
                        loader: serverInfoLoader
                    },
                    {
                        path: "/main/home",
                        element: <Home />
                    },
                    {
                        path: "/main/confirm_logout",
                        element: <ConfirmLogout />
                    }
                ]
            },
            {
                path: "/settings",
                element: <Settings />,
                children: [
                    {
                        path: "/settings/general",
                        element: <General />,
                    },
                    {
                        path: "/settings/parameters",
                        element: <Parameters checkbox={false} />,
                        loader: parametersLoader
                    },
                    {
                        path: "/settings/language",
                        element: <Language />
                    },
                    {
                        path: "/settings/changelog",
                        element: <Changelog />
                    },
                    {
                        path: "/settings/information",
                        element: <Information />
                    }
                ]
            },
            {
                path: "/instanceSettings/:instanceId",
                element: <InstanceSettings />,
                children: [
                    {
                        path: "/instanceSettings/:instanceId/parameters",
                        element: <Parameters checkbox={true} />,
                        loader: parametersLoader
                    },
                    {
                        path: "/instanceSettings/:instanceId/mod_list",
                        element: <ModList />
                    },
                    {
                        path: "/instanceSettings/:instanceId/resource_packs",
                        element: <ResourcePacks />
                    },
                    {
                        path: "/instanceSettings/:instanceId/screenshot",
                        element: <Screenshot />
                    }
                ]
            }
        ]
    }
]);