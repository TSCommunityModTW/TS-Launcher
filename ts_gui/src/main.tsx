import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import { Provider } from "react-redux";

import { store } from "./store";

import routes from "@/routes.tsx";
import "@/i18n.ts";
import "@/styles.scss";

import updater from "./updater";
import logger from "./invoke/logger";

logger.logMessage("debug","main.tsx: loaded")
updater();

ReactDOM
    .createRoot(document.getElementById("root") as HTMLElement)
    .render(
        <Provider store={store}>
            <RouterProvider router={routes} />
        </Provider>
    );