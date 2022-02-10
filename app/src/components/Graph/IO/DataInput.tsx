import clsx from "clsx";
import { Observer, observer } from "mobx-react-lite";
import { FC } from "react";

import { CheckBox, FloatInput, IntInput, TextInput } from "~/components/ui";
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
          onChange={(value) => input.setDefaultValue({ type: "bool", value })}
        />
      );
    }
    case "string": {
      return (
        <div className="w-16">
          <TextInput
            className={className}
            value={input.defaultValue.value}
            onChange={(value) =>
              input.setDefaultValue({ type: "string", value })
            }
          />
        </div>
      );
    }
    case "int": {
      return (
        <div className="w-16">
          <IntInput
            className={className}
            value={input.defaultValue.value}
            onChange={(value) => input.setDefaultValue({ type: "int", value })}
          />
        </div>
      );
    }
    case "float": {
      return (
        <div className="w-16">
          <FloatInput
            className={className}
            value={input.defaultValue.value}
            onChange={(value) =>
              input.setDefaultValue({ type: "float", value })
            }
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
          {input.type.variant === "primitive" && <UnconnectedInput input={input} />}
        </>
      )}
    </Observer>
  </div>
);
