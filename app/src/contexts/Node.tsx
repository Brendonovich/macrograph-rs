import { createContext, FC, useContext } from "react";

import { Node } from "~/models";

const NodeContext = createContext<Node>(null as any);

export const useNode = () => useContext(NodeContext);

export const NodeProvider: FC<{ node: Node }> = ({
  node,
  children,
}) => {
  return <NodeContext.Provider value={node}>{children}</NodeContext.Provider>;
};
