import { invoke } from "@tauri-apps/api";
import "./App.css";

import reactLogo from "../assets/react.svg";
import { ScreenshotList } from "./screenshots/ScreenshotList";

const handleOpenAppDirectory = () => {
  invoke("open_app_directory", { subdirectory: "screenshots" }).catch((err) =>
    console.error("Error opening app directory:", err),
  );
};

export const MainAppContainer = () => {
  return (
    <div className="container flex flex-row gap-8">
      <div className="max-w-[200px]">
        <ScreenshotList />
      </div>

      <div className="flex flex-col items-start">
        <h1>Welcome to Tauri!</h1>

        <div className="flex flex-row">
          <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" className="logo vite" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://reactjs.org" target="_blank">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
        </div>

        <div>
          <button onClick={handleOpenAppDirectory}>
            Open Screenshots Directory
          </button>
        </div>

        <div></div>
      </div>
    </div>
  );
};

