import { useEffect, useRef, useState } from "react";
import { observer } from "mobx-react-lite";
import clsx from "clsx";

import { Node } from "./components/Graph/Node";
import { Core } from "~/models";
import { UI } from "~/stores";
import { ConnectionRender, SchemaMenu } from "./components/Graph";
import { CoreProvider, CurrentGraphProvider } from "./contexts";

const core = new Core();

enum PanState {
  None,
  Waiting,
  Active,
}

function App() {
  const graphRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let lastScale = 1;
    const listener = (e: any) => {
      let scale = e.scale;
      let direction = 1;
      if (scale < 1) {
        direction = -1;
        scale = 1 / scale;
        if (lastScale < 1) {
          lastScale = 1 / lastScale;
        }
      }

      UI.updateScale((scale - lastScale) * direction, {
        x: e.clientX,
        y: e.clientY,
      });

      lastScale = e.scale;
    };

    const resetListener = () => (lastScale = 1);

    graphRef.current?.addEventListener("gesturestart", resetListener);
    graphRef.current?.addEventListener("gesturechange", listener);
    return () => {
      graphRef.current?.removeEventListener("gesturechange", listener);
      graphRef.current?.removeEventListener("gesturechange", resetListener);
    };
  }, [graphRef.current]);

  const [pan, setPan] = useState(PanState.None);

  const [ready, setReady] = useState(false);

  useEffect(() => {
    core.loadPackages().then(() => {
      setReady(true);
      // const keyboardSchema = core.schema("keyboard", "A")!;
      // const obsSchema = core.schema("obs", "Set Current Scene")!;

      // core.graph.createNode(keyboardSchema, { x: 0, y: 0 });
      // core.graph.createNode(obsSchema, { x: 100, y: 100 });
    });
  }, []);
  if (!ready) return <>loading...</>;

  return (
    <CoreProvider core={core}>
      <div className="w-screen h-screen overflow-hidden bg-green-500 select-none">
        <CurrentGraphProvider graph={core.graph}>
          <div
            className={clsx(
              "w-full h-full relative overflow-hidden bg-gray-graph"
            )}
          >
            {UI.schemaMenuPosition && (
              <SchemaMenu
                position={UI.schemaMenuPosition}
                onSchemaClicked={async (s) => {
                  await core.graph.createNode(
                    s,
                    UI.toGraphSpace(UI.schemaMenuPosition!)
                  );
                  UI.setSchemaMenuPosition();
                }}
              />
            )}
            <ConnectionRender />
            <div
              ref={graphRef}
              className={clsx(
                "absolute inset-0 text-white origin-top-left",
                pan === PanState.Active && "cursor-grabbing"
              )}
              style={{
                transform: `scale(${UI.scale})`,
              }}
              onWheel={(e) => {
                let deltaX = e.deltaX,
                  deltaY = e.deltaY,
                  isTouchpad = false;

                if (
                  Math.abs((e.nativeEvent as any).wheelDeltaY) ===
                  Math.abs(e.deltaY) * 3
                ) {
                  deltaX = -(e.nativeEvent as any).wheelDeltaX / 3;
                  deltaY = -(e.nativeEvent as any).wheelDeltaY / 3;
                  isTouchpad = true;
                }

                if (e.ctrlKey) {
                  const delta = ((isTouchpad ? 1 : -1) * deltaY) / 50;
                  UI.updateScale(delta, {
                    x: e.clientX,
                    y: e.clientY,
                  });
                } else {
                  UI.updateTranslate({
                    x: deltaX / UI.scale,
                    y: deltaY / UI.scale,
                  });
                }
              }}
              onMouseUp={(e) => {
                switch (e.button) {
                  case 2:
                    if (pan === PanState.Waiting) {
                      if (UI.mouseDragLocation) UI.setMouseDragLocation();
                      else
                        UI.setSchemaMenuPosition({
                          x: e.clientX,
                          y: e.clientY,
                        });
                    }
                    setPan(PanState.None);
                    break;
                }
              }}
              onMouseDown={(e) => {
                UI.setSchemaMenuPosition();
                switch (e.button) {
                  case 0:
                    UI.setSelectedNode();
                    break;
                  case 2:
                    setPan(PanState.Waiting);
                    UI.setMouseDownLocation({
                      x: e.clientX,
                      y: e.clientY,
                    });
                    UI.setMouseDownTranslate({
                      ...UI.translate,
                    });
                    break;
                }
              }}
              onMouseMove={(e) => {
                if (pan === PanState.None) return;
                if (pan === PanState.Waiting) setPan(PanState.Active);

                UI.setTranslate({
                  x:
                    (UI.mouseDownLocation!.x -
                      e.clientX +
                      UI.mouseDownTranslate!.x * UI.scale) /
                    UI.scale,
                  y:
                    (UI.mouseDownLocation!.y -
                      e.clientY +
                      UI.mouseDownTranslate!.y * UI.scale) /
                    UI.scale,
                });
              }}
              onContextMenu={(e) => {
                e.stopPropagation();
                e.preventDefault();
              }}
            >
              {Object.values(core.graph.nodes).map((node) => (
                <Node node={node} key={node.id} />
              ))}
            </div>
          </div>
        </CurrentGraphProvider>
      </div>
    </CoreProvider>
  );
}

export default observer(App);
