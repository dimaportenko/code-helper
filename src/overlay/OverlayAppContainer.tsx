import { invoke } from "@tauri-apps/api";
import styles from "./App.module.css";
import RectangleSelection, { AreaCoords } from "./selection/RectangleSelection";
import { useHandleEsc } from "./hooks/useHandleEsc";
import { hideCursor, showCursor } from "./utils/cursor";

const onSelectionEnd = async (coords: AreaCoords) => {
  try {
    console.log("coords", coords);
    hideCursor();
    await invoke("screenshot", { coords });
    showCursor();
  } catch (error) {
    console.error(error);
  }
};

export const OverlayAppContainer = () => {
  useHandleEsc();

  return (
    <div id="container" className={styles.container}>
      <RectangleSelection
        onSelectionEnd={onSelectionEnd}
        style={{
          backgroundColor: "rgba(0,0,255,0.4)",
          borderColor: "blue",
        }}
      >
        <div className="App" />
      </RectangleSelection>
    </div>
  );
};
