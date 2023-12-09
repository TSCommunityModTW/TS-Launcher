import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";

import routes from "@/routes.tsx";
import "@/assets/fonts/jf-openhuninn-1.1.ttf";
import "@/i18n.ts";
import "@/styles.scss";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<RouterProvider router={routes} />);