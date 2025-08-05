import type { ComponentPropsWithoutRef } from "react";
import { TrainerButton } from "./TrainerButton";
import { cn } from "@/utlils/cn";

export const TrainerButtons = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"div">) => {
  return (
    <div className={cn("grid grid-cols-4 gap-2", className)} {...rest}>
      <TrainerButton className="row-start-1 row-end-3" value="DoubleOrHit" />
      <TrainerButton value="Hit" />
      <TrainerButton value="Stand" />
      <TrainerButton disabled className="col-span-2" value="Split" />
      <TrainerButton
        className="col-start-4 row-start-1 row-end-3"
        value="DoubleOrStand"
      />
    </div>
  );
};
