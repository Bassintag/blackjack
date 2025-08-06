import type { Rules } from "@blackjack/wasm";
import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface RulesState {
  rules: Rules;
  setRules: (rules: Rules) => void;
  reset: () => void;
}

const defaultRules: Rules = {
  blackjackPayout: "Ratio3to2",
  dealerSoft17: "Stand",
  surrender: "None",
  numDecks: 6,
  maxSplits: 1,
  doubleAfterSplitAllowed: false,
};

export const useRulesState = create(
  persist<RulesState>(
    (set) => ({
      rules: defaultRules,
      setRules: (rules) => set({ rules }),
      reset: () => set({ rules: defaultRules }),
    }),
    { name: "blackjack:rules", version: 2 },
  ),
);
