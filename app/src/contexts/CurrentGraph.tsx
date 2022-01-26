import { createContext, FC, useContext } from "react";

import { Graph } from "~/models";

const CurrentGraphContext = createContext<Graph>(null as any);

export const useCurrentGraph = () => useContext(CurrentGraphContext);

export const CurrentGraphProvider: FC<{ graph: Graph }> = ({
  graph,
  children,
}) => {
  return (
    <CurrentGraphContext.Provider value={graph}>
      {children}
    </CurrentGraphContext.Provider>
  );
};
