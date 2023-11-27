import { useState } from "react";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";

export type ScreenshotItem = {
  filePath: string;
  assetPath: string;
};

export const useScreenshotList = () => {
  const [screenshots, setScreenshots] = useState<ScreenshotItem[]>([]);
  const getScreenshotsData = async () => {
    console.log("Getting file paths...");
    invoke("get_screenshot_files")
      .then((files) => {
        const fileSrcs = (files as string[]).map((file) => {
          return {
            filePath: file,
            assetPath: convertFileSrc(file),
          };
        });
        setScreenshots(fileSrcs);
      })
      .catch((error) =>
        console.error("Failed to get screenshot files:", error),
      );
  };

  return {
    screenshots,
    getScreenshotsData,
  };
};
