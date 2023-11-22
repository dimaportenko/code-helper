import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

const rootElement = document.getElementById("root") as HTMLElement;

const queryParams = new URLSearchParams(window.location.search);
const window_id = queryParams.get('window_id');


ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <App windowId={window_id} />
  </React.StrictMode>
);
