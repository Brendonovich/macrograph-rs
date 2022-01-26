import { Observer } from "mobx-react-lite";
import { FC } from "react";

import { DataOutput as DataOutputModel } from "~/models";
import { DataPin } from "./DataPin";

interface Props {
  output: DataOutputModel;
}

export const DataOutput: FC<Props> = ({ output }) => {
  return (
    <div className="flex flex-row items-center space-x-1.5 h-5">
      <Observer>
        {() => (
          <>
            <span>{output.name}</span>
          </>
        )}
      </Observer>
      <DataPin pin={output} />
    </div>
  );
};
