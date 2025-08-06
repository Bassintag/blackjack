import type { PlayerAction } from "@blackjack/wasm";
import type { Color } from "./color";

const configs: Record<
  PlayerAction,
  { label: string; short: string; color: Color }
> = {
  ["Hit"]: { label: "Hit", short: "H", color: "red" },
  ["Stand"]: { label: "Stand", short: "S", color: "yellow" },
  ["DoubleOrHit"]: { label: "Double (or hit)", short: "Dh", color: "blue" },
  ["DoubleOrStand"]: { label: "Double (or stand)", short: "Ds", color: "blue" },
  ["Split"]: { label: "Split", short: "P", color: "green" },
  ["Surrender"]: { label: "Surrender", short: "SUR", color: "purple" },
};

export const getPlayerActionConfig = (action: PlayerAction) => {
  return configs[action];
};
