import { appWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";
import { classNames } from "../../utils/classNames";
import { ScreenshotItem, useScreenshotList } from "./useScreenshotList";

type Props = {
  onSelectedChange: (imageSrc: ScreenshotItem | undefined) => void;
};


export const ScreenshotList = ({ onSelectedChange }: Props) => {
  const [selected, setSelected] = useState<number>(0); // index of filePaths[
  const onFirstLoad = useRef(false);

  const {screenshots, getScreenshotsData} = useScreenshotList();

  useEffect(() => {
    let unlisten = () => {};
    appWindow
      .onFocusChanged(({ payload: focused }) => {
        if (focused) {
          getScreenshotsData();
        }
      })
      .then((fn) => (unlisten = fn));

    getScreenshotsData();

    return () => {
      unlisten();
    };
  }, []);

  useEffect(() => {
    let unsibscribe = () => {};
    listen("on_screenshot", () => {
      getScreenshotsData().then(() => {
        setSelected(0);
        onSelectedChange(screenshots[0]);
      });
      onFirstLoad.current = false;
    }).then((fn) => (unsibscribe = fn));

    return () => {
      unsibscribe();
    };
  }, []);

  useEffect(() => {
    if (screenshots.length) {
      if (!onFirstLoad.current) {
        setSelected(0);
        onSelectedChange(screenshots[0]);
        onFirstLoad.current = true;
      }
    }
  }, [screenshots]);

  return (
    <div className="flex flex-col gap-4">
      {screenshots.map((screenshot, index) => (
        <div
          onClick={() => {
            setSelected(index);
            onSelectedChange(screenshot);
          }}
          key={index}
          className={classNames(
            selected === index ? "border-blue-500 border" : "",
          )}
        >
          <img src={screenshot.assetPath} alt="Screenshot" width={200} />
          {/* <span className="w-[200px] max-w-[200px]">{file}</span> */}
        </div>
      ))}
    </div>
  );
};
