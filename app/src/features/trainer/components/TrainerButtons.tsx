import type { ComponentPropsWithoutRef } from "react";
import { TrainerButton } from "./TrainerButton";
import { cn } from "@/utlils/cn";
import { useRulesState } from "@/features/rules/hooks/useRulesState";

export const TrainerButtons = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"div">) => {
  const rules = useRulesState((s) => s.rules);
  return (
    <div
      className={cn("grid grid-cols-6 gap-2 max-w-128 mx-auto", className)}
      {...rest}
    >
      <TrainerButton className="col-span-3" value="Hit" />
      <TrainerButton className="col-span-3" value="Stand" />
      <TrainerButton className="col-span-2" value="DoubleOrHit" />
      <TrainerButton className="col-span-2" value="Split" />
      <TrainerButton className="col-span-2" value="DoubleOrStand" />
      {rules.surrender != "None" && (
        <TrainerButton className="col-span-6" value="Surrender" />
      )}
    </div>
  );
};
