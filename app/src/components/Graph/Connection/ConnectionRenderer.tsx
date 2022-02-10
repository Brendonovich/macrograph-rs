import { observer } from "mobx-react-lite";

import { UI } from "~/stores";
import { useCurrentGraph } from "~/contexts";
import { DataInput, DataOutput } from "~/models";
import { ReactNode } from "react";
import { PrimitiveType, Value } from "@macrograph/core-types";

const DataColourClasses: Record<PrimitiveType, string> = {
  bool: "text-red-bool",
  string: "text-pink-string",
  float: "text-green-float",
  int: "text-blue-int",
};

export const ConnectionRender = observer(() => {
  const graph = useCurrentGraph();

  let mouseConnection: ReactNode = null;
  if (UI.draggingPin && UI.mouseDragLocation) {
    let pinPos = UI.pinPositions.get(UI.draggingPin);
    if (pinPos) {
      let colourClass = "text-white";

      if (
        UI.draggingPin instanceof DataInput ||
        UI.draggingPin instanceof DataOutput
      )
        if (UI.draggingPin.type.variant === "primitive") {
          colourClass = DataColourClasses[UI.draggingPin.type.value];
        } else {
          colourClass = DataColourClasses[UI.draggingPin.type.value.value];
        }

      mouseConnection = (
        <line
          className={colourClass}
          x1={pinPos.x - UI.graphOffset.x}
          y1={pinPos.y - UI.graphOffset.y}
          x2={UI.mouseDragLocation.x - UI.graphOffset.x}
          y2={UI.mouseDragLocation.y - UI.graphOffset.y}
          stroke="currentColor"
          strokeOpacity={0.75}
          strokeWidth={2 * UI.scale}
        />
      );
    }
  }

  return (
    <svg className="w-full h-full transform">
      <g>
        {Object.values(graph.nodes).map((n) =>
          n.inputs.map((i) => {
            if (!i.connected) return null;

            const key = `${n.id}-${i.name}`;

            const inputPos = UI.pinPositions.get(i);
            if (!inputPos) return null;

            if (i instanceof DataInput) {
              const outputPos = UI.pinPositions.get(i.connection!);
              if (!outputPos) return null;

              let colourClass;
              if (i.type.variant === "primitive") {
                colourClass = DataColourClasses[i.type.value];
              } else {
                colourClass = DataColourClasses[i.type.value.value];
              }

              return (
                <line
                  key={key}
                  className={colourClass}
                  x1={inputPos.x - UI.graphOffset.x}
                  y1={inputPos.y - UI.graphOffset.y}
                  x2={outputPos.x - UI.graphOffset.x}
                  y2={outputPos.y - UI.graphOffset.y}
                  stroke="currentColor"
                  strokeOpacity={0.75}
                  strokeWidth={2 * UI.scale}
                />
              );
            } else {
              const outputPos = UI.pinPositions.get(i.connection!);
              if (!outputPos) return null;

              return (
                <line
                  key={key}
                  x1={inputPos.x - UI.graphOffset.x}
                  y1={inputPos.y - UI.graphOffset.y}
                  x2={outputPos.x - UI.graphOffset.x}
                  y2={outputPos.y - UI.graphOffset.y}
                  stroke={"white"}
                  strokeOpacity={0.75}
                  strokeWidth={2 * UI.scale}
                />
              );
            }
          })
        )}
        {mouseConnection}
      </g>
    </svg>
  );
});
