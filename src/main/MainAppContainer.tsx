import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import "./App.css";

import reactLogo from "../assets/react.svg";
import { useEffect, useState } from "react";

const handleOpenAppDirectory = () => {
  invoke("open_app_directory", { subdirectory: "screenshots" }).catch((err) =>
    console.error("Error opening app directory:", err),
  );
};

export const MainAppContainer = () => {
  const [filePaths, setFilePaths] = useState<string[]>([]);

  useEffect(() => {
    invoke("get_screenshot_files")
      .then((files) => {
        const fileSrcs = (files as string[]).map((file) => convertFileSrc(file))
        setFilePaths(fileSrcs)
      })
      .catch((error) =>
        console.error("Failed to get screenshot files:", error),
      );
  }, []);

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
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

      <div>
        <ul>
          {filePaths.map((file, index) => (
            <li key={index}>
              <img src={file} alt="Screenshot" width={200} />
              <span>{file}</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
