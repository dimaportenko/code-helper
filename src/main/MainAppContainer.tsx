import { invoke } from "@tauri-apps/api";
import "./App.css";

import { ScreenshotList } from "./screenshots/ScreenshotList";
import { useState } from "react";
import { ScreenshotItem } from "./screenshots/useScreenshotList";

const handleOpenAppDirectory = () => {
  invoke("open_app_directory", { subdirectory: "screenshots" }).catch((err) =>
    console.error("Error opening app directory:", err),
  );
};

const generateCode = (filePath: string | undefined) => {
  if (!filePath) {
    alert("No screenshot selected!");
    return;
  }

  console.log("Generating code for:", filePath);
  invoke("image_to_code", { filePath }).catch((err) =>
    console.error("Error: Can't generate code:", err),
  );
};

export const MainAppContainer = () => {
  const [selectedScreenshot, setSelectedScreenshot] = useState<
    ScreenshotItem | undefined
  >();

  return (
    <div className="container flex flex-row gap-8">
      <div className="flex flex-col max-w-[200px] p-1 border-r border-r-gray-500/30 h-full">
        <div className="flex flex-col flex-1 overflow-y-auto">
          <ScreenshotList
            onSelectedChange={(screenshot: ScreenshotItem | undefined) => {
              setSelectedScreenshot(screenshot);
            }}
          />
        </div>

        <div className="flex justify-center p-1 pt-2">
          <button onClick={handleOpenAppDirectory}>Open Screenshots</button>
        </div>
      </div>

      <div className="flex flex-col items-start">
        <h1>{selectedScreenshot?.assetPath.split("%2F").at(-1)}</h1>

        <div className="flex flex-row">
          <img
            src={selectedScreenshot?.assetPath}
            alt="Screenshot"
            width={500}
          />
        </div>

        <div className="flex justify-center p-1 pt-2">
          <button onClick={() => generateCode(selectedScreenshot?.filePath)}>
            Generate Code
          </button>
        </div>
      </div>
    </div>
  );
};
