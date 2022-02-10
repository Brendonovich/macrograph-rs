import { useEffect, useState } from "react";
import { observer } from "mobx-react-lite";

import { Core } from "~/models";
import { CoreProvider, CurrentGraphProvider } from "./contexts";
import { Graph } from "~/components/Graph";
import { GraphList } from "~/components/ProjectSidebar";
import { UI } from "./stores";

const core = new Core();

function App() {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    (async () => {
      await core.loadPackages();
      await core.loadProject();
      UI.setCurrentGraph(Object.values(core.graphs)[0]);
      setReady(true);
    })();
  }, []);
  if (!ready) return <>loading...</>;

  return (
    <CoreProvider core={core}>
      <div className="w-screen h-screen flex flex-row overflow-hidden select-none">
        <GraphList
          currentGraph={UI.currentGraph}
          onChange={(g) => UI.setCurrentGraph(g)}
          graphs={Object.values(core.graphs)}
        />
        {UI.currentGraph && (
          <CurrentGraphProvider graph={UI.currentGraph}>
            <Graph />
          </CurrentGraphProvider>
        )}
      </div>
    </CoreProvider>
  );
}

export default observer(App);
