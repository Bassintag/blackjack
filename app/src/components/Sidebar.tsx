import { cn } from "@/utlils/cn";
import type { ComponentPropsWithoutRef } from "react";
import { Card } from "./Card";

export const Sidebar = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<typeof Card>) => {
  return (
    <Card
      className={cn(
        "fixed left-3 top-3 bottom-3 flex flex-col w-64",
        className,
      )}
      {...rest}
    />
  );
};
