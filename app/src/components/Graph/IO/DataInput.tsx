import clsx from "clsx";
import { Observer, observer } from "mobx-react-lite";
import { FC } from "react";

import { CheckBox, TextInput } from "~/components/ui";
import { DataInput as DataInputModel } from "~/models";
import { DataPin } from ".";

const UnconnectedInput = observer<Props>(({ input }) => {
  const connected = input.connection !== null;
  const className = clsx(connected && "opacity-0 pointer-events-none");

  switch (input.defaultValue.type) {
    case "bool": {
      return (
        <CheckBox
          className={className}
          value={input.defaultValue.value}
          onChange={(v) => input.setDefaultValue(v)}
        />
      );
    }
    case "string": {
      return (
        <div className="w-16">
          <TextInput
            className={className}
            value={input.defaultValue.value}
            onChange={(v) => input.setDefaultValue(v)}
          />
        </div>
      );
    }
  }
});

interface Props {
  input: DataInputModel;
}

export const DataInput: FC<Props> = ({ input }) => (
  <div className="flex flex-row items-center space-x-1.5 h-5">
    <DataPin pin={input} />
    <Observer>
      {() => (
        <>
          <span>{input.name}</span>
          <UnconnectedInput input={input} />
        </>
      )}
    </Observer>
  </div>
);
