import type { PlayerAction } from "@blackjack/wasm";
import type { Color } from "./color";

const configs: Record<
  PlayerAction,
  { label: string; short: string; color: Color }
> = {
  ["Hit"]: { label: "Hit", short: "H", color: "red" },
  ["Stand"]: { label: "Stand", short: "S", color: "yellow" },
  ["DoubleOrHit"]: { label: "Double (or hit)", short: "DH", color: "blue" },
  ["DoubleOrStand"]: { label: "Double (or stand)", short: "DS", color: "blue" },
  ["Split"]: { label: "Split", short: "P", color: "green" },
};

export const getPlayerActionConfig = (action: PlayerAction) => {
  return configs[action];
};
