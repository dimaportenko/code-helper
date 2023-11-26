import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export const ScreenshotList = () => {
  const [filePaths, setFilePaths] = useState<string[]>([]);

  useEffect(() => {
    invoke("get_screenshot_files")
      .then((files) => {
        const fileSrcs = (files as string[]).map((file) =>
          convertFileSrc(file),
        );
        setFilePaths(fileSrcs);
      })
      .catch((error) =>
        console.error("Failed to get screenshot files:", error),
      );
  }, []);

  return (
    <div className="flex flex-col gap-8">
      {filePaths.map((file, index) => (
        <div key={index}>
          <img src={file} alt="Screenshot" width={200} />
          {/* <span className="w-[200px] max-w-[200px]">{file}</span> */}
        </div>
      ))}
    </div>
  );
};
