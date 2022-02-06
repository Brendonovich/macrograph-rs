import { useEffect, useRef, useState } from "react";
import clsx from "clsx";

import { Node } from "./Node";
import { UI } from "~/stores";
import { ConnectionRender, SchemaMenu } from "~/components/Graph";
import { useCurrentGraph } from "~/contexts";
import { observer } from "mobx-react-lite";

enum PanState {
  None,
  Waiting,
  Active,
}

export const Graph = observer(() => {
  const graph = useCurrentGraph();

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

  return (
    <div
      className={clsx("w-full h-full relative overflow-hidden bg-gray-graph")}
    >
      {UI.schemaMenuPosition && (
        <SchemaMenu
          position={UI.schemaMenuPosition}
          onSchemaClicked={async (s) => {
            await graph.createNode(s, UI.toGraphSpace(UI.schemaMenuPosition!));
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
        {Object.values(graph.nodes).map((node) => (
          <Node node={node} key={node.id} />
        ))}
      </div>
    </div>
  );
});
