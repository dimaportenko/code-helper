import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

const rootElement = document.getElementById("root") as HTMLElement;

const windowId = rootElement.dataset.windowId;

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <App windowId={windowId} />
  </React.StrictMode>
);
