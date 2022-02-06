import clsx from "clsx";
import { FC } from "react";

import { DataInput, DataOutput } from "~/models";
import { usePin } from "~/hooks";
import { Observer } from "mobx-react-lite";
import { Value } from "@macrograph/core-types";

const DataPinTypeColours: Record<
  Value["type"],
  { active: string; base: string }
> = {
  bool: {
    active: "border-red-bool bg-red-bool",
    base: "border-red-bool hover:bg-red-bool",
  },
  string: {
    active: "border-pink-string bg-pink-string",
    base: "border-pink-string hover:bg-pink-string",
  },
  int: {
    active: "border-int-blue bg-int-blue",
    base: "border-int-blue hover:bg-int-blue",
  },
  float: {
    active: "border-green-float bg-green-float",
    base: "border-green-float hover:bg-green-float",
  },
};

interface Props {
  pin: DataInput | DataOutput;
}

export const DataPin: FC<Props> = ({ pin }) => {
  const colourClass = DataPinTypeColours[pin.type.type];

  const { ref, active } = usePin(pin);

  return (
    <Observer>
      {() => (
        <div
          ref={ref}
          style={{
            pointerEvents: "all",
          }}
          className={clsx(
            `w-3.5 h-3.5 rounded-full border-2`,
            pin.connected || active ? colourClass.active : colourClass.base
          )}
        />
      )}
    </Observer>
  );
};
