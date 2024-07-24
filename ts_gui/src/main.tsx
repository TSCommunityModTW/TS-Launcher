import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import { Provider } from "react-redux";

import { store } from "./store";

import routes from "@/routes.tsx";
import "@/i18n.ts";
import "@/styles.scss";



ReactDOM
    .createRoot(document.getElementById("root") as HTMLElement)
    .render(
        <Provider store={store}>
            <RouterProvider router={routes} />
        </Provider>
    );