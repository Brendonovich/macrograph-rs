import clsx from "clsx";
import { FC } from "react";

import { DataInput, DataOutput } from "~/models";
import { usePin } from "~/hooks";
import { Observer } from "mobx-react-lite";
import { PrimitiveType, Value } from "@macrograph/core-types";

const DataPinTypeColours: Record<
  PrimitiveType,
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
    active: "border-blue-int bg-blue-int",
    base: "border-blue-int hover:bg-blue-int",
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
  let type: PrimitiveType;
  let isArray = false;
  if (pin.type.variant === "primitive") {
    type = pin.type.value;
  } else {
    isArray = true;
    if (pin.type.value.variant === "primitive") type = pin.type.value.value;
    else throw "";
  }

  let colourClass = DataPinTypeColours[type];

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
            `w-3.5 h-3.5 border-2`,
            !isArray && "rounded-full",
            pin.connected || active ? colourClass.active : colourClass.base
          )}
        />
      )}
    </Observer>
  );
};
