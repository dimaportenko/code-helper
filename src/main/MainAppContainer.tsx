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

const generateCode = (
  filePath: string | undefined,
  successCallback: (response: string) => void,
) => {
  if (!filePath) {
    alert("No screenshot selected!");
    return;
  }

  console.log("Generating code for:", filePath);
  invoke("image_to_code", { filePath })
    .then((response) => {
      successCallback(response as string);
      console.log("Response:", response);
    })
    .catch((err) => console.error("Error: ", err));
};

export const MainAppContainer = () => {
  const [response, setResponse] = useState<string | undefined>("");
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

      <div className="flex flex-col items-start gap-6 max-w-[50%] overflow-y-auto">

        <h1>{selectedScreenshot?.assetPath.split("%2F").at(-1)}</h1>

        <div className="flex flex-row">
          <img
            src={selectedScreenshot?.assetPath}
            alt="Screenshot"
            className="w-[300px] h-[300px] object-contain"
          />
        </div>

        <div className="flex flex-col justify-center p-1 pt-2 max-w-[50%]">
          <button
            onClick={() =>
              generateCode(selectedScreenshot?.filePath, (response) => {
                setResponse(response);
              })
            }
          >
            Generate Code
          </button>
        </div>

        <div className="max-w-[50%]">
          <pre>
            <div className="break-words">{response}</div>
          </pre>
        </div>
      </div>

      <div id="preview flex-1">
        <div dangerouslySetInnerHTML={{__html: response ?? ""}}></div>
      </div>
    </div>
  );
};
