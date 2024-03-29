import clsx from "clsx";
import { observer } from "mobx-react-lite";
import { NodeSchemaType } from "@macrograph/core-types";
import { useCallback } from "react";

import { NodeProvider, useCurrentGraph } from "~/contexts";
import {
  Node as NodeModel,
  DataInput as DataInputModel,
  DataOutput as DataOutputModel,
} from "~/models";
import { UI } from "~/stores";
import { DataInput, DataOutput, ExecInput, ExecOutput } from "./IO";

interface Props {
  node: NodeModel;
}

const SchemaTypeColours: Record<NodeSchemaType, string> = {
  Exec: "bg-blue-exec",
  Base: "bg-gray-base",
  Event: "bg-red-event",
  Pure: "bg-green-pure",
};

export const Node = observer<Props>(({ node }) => {
  const graph = useCurrentGraph();
  const handleMouseMove = useCallback(
    (e: MouseEvent) => {
      node.setPosition({
        x: node.position.x + e.movementX / UI.scale,
        y: node.position.y + e.movementY / UI.scale,
      });
    },
    [node]
  );

  return (
    <NodeProvider node={node}>
      <div
        className={clsx(
          "absolute top-0 left-0 text-[12px] overflow-hidden rounded-lg flex flex-col bg-black/75 border-black/75 border-2",
          node.selected && "ring-2 ring-yellow-500"
        )}
        style={{
          transform: `translate(${node.position.x - UI.translate.x}px, ${
            node.position.y - UI.translate.y
          }px)`,
        }}
      >
        <div
          className={clsx(
            "h-6 px-2 pt-1 text-md font-medium cursor-pointer outline-none",
            SchemaTypeColours[node.schema.type]
          )}
          onKeyDown={(e) => {
            switch (e.key) {
              case "Delete": {
                graph.deleteNode(node.id);
                break;
              }
            }
          }}
          tabIndex={-1}
          onMouseDown={(e) => {
            e.currentTarget.focus();
            e.stopPropagation();
            e.preventDefault();
            switch (e.button) {
              case 0: {
                UI.setSelectedNode(node);

                window.addEventListener("mousemove", handleMouseMove);
                const listener = () => {
                  window.removeEventListener("mouseup", listener);
                  window.removeEventListener("mousemove", handleMouseMove);
                };
                window.addEventListener("mouseup", listener);

                break;
              }
              default:
                break;
            }
          }}
          onContextMenu={(e) => {
            e.preventDefault();
            e.stopPropagation();
          }}
        >
          {node.schema.name}
        </div>
        <div className="flex flex-row gap-2">
          <div className="p-2 flex flex-col space-y-2.5">
            {node.inputs.map((i) =>
              i instanceof DataInputModel ? (
                <DataInput input={i} key={i.name} />
              ) : (
                <ExecInput input={i} key={i.name} />
              )
            )}
          </div>
          <div className="p-2 ml-auto flex flex-col space-y-2.5 items-end">
            {node.outputs.map((o) =>
              o instanceof DataOutputModel ? (
                <DataOutput output={o} key={o.name} />
              ) : (
                <ExecOutput output={o} key={o.name} />
              )
            )}
          </div>
        </div>
      </div>
    </NodeProvider>
  );
});
