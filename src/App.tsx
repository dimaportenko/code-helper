import { OverlayAppContainer } from "./overlay/OverlayAppContainer";
import { MainAppContainer } from "./main/MainAppContainer";

type Props = {
  windowId?: string;
};

function App({ windowId }: Props) {
  if (windowId === "overlay") {
    return <OverlayAppContainer />;
  }

  return <MainAppContainer />;
}

export default App;
