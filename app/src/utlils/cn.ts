import clsx from "clsx";
import { twMerge } from "tailwind-merge";

export const cn = (...input: Parameters<typeof clsx>) => {
  return twMerge(clsx(...input));
};
