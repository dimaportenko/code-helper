import styles from "./App.module.css";
import { invoke } from "@tauri-apps/api";

async function doScreenshot() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("screenshot");
}

export const OverlayAppContainer = () => {
  return (
    <div className={styles.container}>
      <button onClick={doScreenshot}>Screenshot</button>
    </div>
  );
};
