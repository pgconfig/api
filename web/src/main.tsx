import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";
import { Toasts } from "@momoi-labs/kiso-react";

import "@momoi-labs/kiso-react/styles.css";
import "./app.css";
import { App } from "./App.js";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Toasts>
        <App />
      </Toasts>
    </BrowserRouter>
  </StrictMode>,
);
