import { Button } from "@/components/Button";
import { cn } from "@/utlils/cn";
import { backgroundColor } from "@/utlils/color";
import { getPlayerActionConfig } from "@/utlils/playerAction";
import type { PlayerAction } from "@blackjack/wasm";
import type { ComponentPropsWithoutRef } from "react";
import { useTrainerState } from "../hooks/useTrainerState";
import { useStrategyGenerator } from "@/contexts/StrategyGeneratorContext";
import { asRank } from "@/utlils/card";

export interface TrainerButtonProps extends ComponentPropsWithoutRef<"button"> {
  value: PlayerAction;
}

export const TrainerButton = ({
  value,
  className,
  ...rest
}: TrainerButtonProps) => {
  const { label, color } = getPlayerActionConfig(value);
  const { dealer, player, roll } = useTrainerState();
  const strategyGenerator = useStrategyGenerator();

  return (
    <Button
      className={cn(
        "font-semibold text-white h-auto",
        backgroundColor(color),
        className,
      )}
      onClick={() => {
        const correctValue = strategyGenerator.action(
          player.map(asRank),
          asRank(dealer),
        );
        roll(value === correctValue);
      }}
      {...rest}
    >
      {label}
    </Button>
  );
};
