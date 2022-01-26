import clsx from "clsx";
import { FC } from "react";

interface Props {
  value: boolean;
  onChange(value: boolean): void;
  className?: string;
}

export const CheckBox: FC<Props> = ({ value, onChange, className }) => {
  return (
    <label className={clsx("w-4 h-4 cursor-pointer relative", className)}>
      <input
        type="checkbox"
        onChange={(e) => onChange(e.target.checked)}
        checked={value}
        className={clsx("absolute opacity-0 w-0 h-0 peer")}
      />
      <span className="absolute inset-0 peer-checked:bg-[#0075FF] peer-checked:border-[#0075FF] bg-white border-gray-500 rounded border" />
      <svg
        viewBox="0 0 13 10"
        fill="transparent"
        className="absolute w-3 h-3 top-0.5 left-0.5"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M1.5 5.5L4.5 8.5L10.5 1.5" stroke="white" strokeWidth="2" />
      </svg>
    </label>
  );
};
