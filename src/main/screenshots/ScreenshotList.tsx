import { appWindow } from "@tauri-apps/api/window";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { classNames } from "../../utils/classNames";

type Props = {
  onSelectedChange: (imageSrc: string | undefined) => void;
};

export const ScreenshotList = ({ onSelectedChange }: Props) => {
  const [selected, setSelected] = useState<number>(0); // index of filePaths[
  const [filePaths, setFilePaths] = useState<string[]>([]);

  useEffect(() => {
    const getFilePaths = async () => {
      console.log("Getting file paths...");
      invoke("get_screenshot_files")
        .then((files) => {
          const fileSrcs = (files as string[]).map((file) =>
            convertFileSrc(file),
          );
          if (fileSrcs.length > 0) {
            onSelectedChange(fileSrcs[0]);
          }
          setFilePaths(fileSrcs);
        })
        .catch((error) =>
          console.error("Failed to get screenshot files:", error),
        );
    };

    let unlisten = () => {};
    appWindow
      .onFocusChanged(({ payload: focused }) => {
        if (focused) {
          getFilePaths();
        }
      })
      .then((fn) => (unlisten = fn));

    getFilePaths();

    return () => {
      unlisten();
    };
  }, []);

  return (
    <div className="flex flex-col gap-4">
      {filePaths.map((file, index) => (
        <div
          onClick={() => {
            setSelected(index);
            onSelectedChange(file);
          }}
          key={index}
          className={classNames(
            selected === index ? "border-blue-500 border" : "",
          )}
        >
          <img src={file} alt="Screenshot" width={200} />
          {/* <span className="w-[200px] max-w-[200px]">{file}</span> */}
        </div>
      ))}
    </div>
  );
};
