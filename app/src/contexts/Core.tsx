import { createContext, FC, useContext } from "react";

import { Core } from "~/models";

const CoreContext = createContext<Core>(null as any);

export const useCore = () => useContext(CoreContext);

export const CoreProvider: FC<{ core: Core }> = ({ core, children }) => {
  return <CoreContext.Provider value={core}>{children}</CoreContext.Provider>;
};
