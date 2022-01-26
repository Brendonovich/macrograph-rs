import clsx from "clsx";
import { FC } from "react";

interface Props {
  value: string;
  onChange(v: string): void;
  className?: string;
}

export const TextInput: FC<Props> = ({ value, onChange, className }) => (
  <input
    type="text"
    value={value}
    onChange={(e) => onChange(e.target.value)}
    className={clsx(
      "w-full text-xs h-5 px-1 border border-gray-300 rounded bg-black focus:border-yellow-500 focus:ring-0",
      className
    )}
  />
);
