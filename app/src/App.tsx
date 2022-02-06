import { useEffect, useState } from "react";
import { observer } from "mobx-react-lite";

import { Core } from "~/models";
import { CoreProvider, CurrentGraphProvider } from "./contexts";
import { Graph } from "~/components/Graph";

const core = new Core();

function App() {
  const [ready, setReady] = useState(false);
  const [currentGraph, setCurrentGraph] = useState(core.graphs[0]);

  useEffect(() => {
    (async () => {
      await core.loadPackages();
      await core.loadProject();
      setCurrentGraph(core.graphs[0]);
      setReady(true);
    })();
  }, []);
  if (!ready) return <>loading...</>;

  return (
    <CoreProvider core={core}>
      <div className="w-screen h-screen overflow-hidden bg-green-500 select-none">
        <CurrentGraphProvider graph={currentGraph}>
          <Graph />
        </CurrentGraphProvider>
      </div>
    </CoreProvider>
  );
}

export default observer(App);
