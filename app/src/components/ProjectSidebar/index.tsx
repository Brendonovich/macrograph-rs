import { observer } from "mobx-react-lite";
import { useCore } from "~/contexts";
import { Graph } from "~/models";
import { UI } from "~/stores";
import { GraphItem } from "./GraphItem";

// React component to show a list of projects
interface Props {
  graphs: Graph[];
  currentGraph: Graph | null;
  onChange: (graph: Graph) => void;
}

export const GraphList = observer<Props>(
  ({ graphs, onChange, currentGraph }) => {
    const core = useCore();

    return (
      <div className="flex flex-col w-80 bg-neutral-600 shadow-2xl">
        <div className="flex flex-row bg-neutral-900 text-white px-2 font-medium shadow">
          <div className="flex-1 py-1">Graphs</div>
          <button
            className="text-xl font-bold"
            onClick={async () => {
              let graph = await core.createGraph();
              UI.setCurrentGraph(graph);
            }}
          >
            +
          </button>
        </div>
        {graphs.map((graph) => (
          <GraphItem
            key={graph.id}
            graph={graph}
            onClick={() => onChange(graph)}
            isCurrentGraph={graph === currentGraph}
          />
        ))}
      </div>
    );
  }
);
