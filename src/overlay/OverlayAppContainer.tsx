import styles from "./App.module.css";
import { invoke } from "@tauri-apps/api";
import RectangleSelection from "./selection/RectangleSelection";

export const OverlayAppContainer = () => {
  return (
    <div className={styles.container}>
      <RectangleSelection
        onSelectionEnd={(coords) => {
          console.log("coords", coords);
          invoke("screenshot", { coords });
        }}
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
