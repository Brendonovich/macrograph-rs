import clsx from "clsx";
import { FC, useCallback, useEffect, useState } from "react";

interface Props {
  value: number;
  onChange(v: number): void;
  className?: string;
}

// floating point regex
const FLOAT_REGEX = /^[+-]?\d*(\.\d+)?$/;

export const FloatInput: FC<Props> = ({
  value: currentValue,
  onChange,
  className,
}) => {
  const [value, setValue] = useState(currentValue.toString());

  const handleChange = useCallback(
    (value: string) => {
      setValue(value);

      if (FLOAT_REGEX.test(value)) onChange(parseInt(value));
    },
    [setValue, onChange]
  );

  return (
    <input
      type="text"
      value={value}
      onChange={(e) => handleChange(e.target.value)}
      onBlur={(e) => {
        if (e.target.value.length === 0) {
          setValue("0");
          onChange(0);
        } else if (!FLOAT_REGEX.test(e.target.value)) {
          setValue(currentValue.toString());
        }
      }}
      className={clsx(
        "w-full text-xs h-5 px-1 border border-gray-300 rounded bg-black focus:border-yellow-500 focus:ring-0",
        className
      )}
    />
  );
};
