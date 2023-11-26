import { invoke } from "@tauri-apps/api";
import "./App.css";

import { ScreenshotList } from "./screenshots/ScreenshotList";
import { useState } from "react";

const handleOpenAppDirectory = () => {
  invoke("open_app_directory", { subdirectory: "screenshots" }).catch((err) =>
    console.error("Error opening app directory:", err),
  );
};

export const MainAppContainer = () => {
  const [selectedScreenshot, setSelectedScreenshot] = useState<
    string | undefined
  >();

  return (
    <div className="container flex flex-row gap-8">
      <div className="flex flex-col max-w-[200px] p-1 border-r border-r-gray-500/30 h-full">
        <div className="flex flex-col flex-1 overflow-y-auto">
          <ScreenshotList
            onSelectedChange={(imageSrc) => {
              setSelectedScreenshot(imageSrc);
            }}
          />
        </div>

        <div className="flex justify-center p-1 pt-2">
          <button onClick={handleOpenAppDirectory}>Open Screenshots</button>
        </div>
      </div>

      <div className="flex flex-col items-start">
        <h1>{selectedScreenshot?.split("%2F").at(-1)}</h1>

        <div className="flex flex-row">
          <img src={selectedScreenshot} alt="Screenshot" width={500} />
        </div>

        <div></div>

        <div></div>
      </div>
    </div>
  );
};
