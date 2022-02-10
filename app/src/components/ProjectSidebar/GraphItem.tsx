// react component

import clsx from "clsx";
import { observer } from "mobx-react-lite";
import { useEffect, useState } from "react";

import { Graph } from "~/models";

interface Props {
  graph: Graph;
  onClick: () => void;
  isCurrentGraph: boolean;
}

export const GraphItem = observer<Props>(
  ({ graph, onClick, isCurrentGraph }) => {
    const [editing, setEditing] = useState(isCurrentGraph);

    useEffect(() => {
      setEditing(false);
    }, [isCurrentGraph]);

    return (
      <div
        className={clsx(
          "cursor-pointer text-white",
          isCurrentGraph ? "bg-neutral-700" : "hover:bg-neutral-500"
        )}
      >
        {!editing ? (
          <div
            className="flex flex-row items-center px-2 py-1 w-full border-2 border-transparent"
            onClick={onClick}
            onDoubleClick={() => setEditing(true)}
          >
            {graph.name}
          </div>
        ) : (
          <input
            className={clsx(
              "px-2 py-1 w-full outline-none box-border border-2 border-sky-600",
              isCurrentGraph ? "bg-neutral-700" : "hover:bg-neutral-500"
            )}
            autoFocus
            value={graph.name}
            onChange={(e) => graph.rename(e.target.value)}
            onKeyPress={(e) => setEditing(false)}
          />
        )}
      </div>
    );
  }
);
