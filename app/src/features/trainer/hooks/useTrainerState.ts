import { CardColor, CardRank, type CardValue } from "@/domain/CardValue";
import { randomPick } from "@/utlils/array";
import { create } from "zustand";

export interface TrainerState {
  player: CardValue[];
  dealer: CardValue;

  stats: {
    total: number;
    success: number;
    failures: number;
  };

  roll(success: boolean): void;
}

const colors = Object.values(CardColor).filter((v) => typeof v === "number");
const ranks = Object.values(CardRank).filter((v) => typeof v === "number");

export const useTrainerState = create<TrainerState>((set, get) => {
  const randomCard = (): CardValue => {
    const color = randomPick(colors);
    const rank = randomPick(ranks);
    return { color, rank };
  };

  return {
    player: [randomCard(), randomCard()],
    dealer: randomCard(),
    stats: { total: 0, success: 0, failures: 0 },
    roll: (success) => {
      const { stats } = get();
      stats.total += 1;
      if (success) {
        stats.success += 1;
      } else {
        stats.failures += 1;
      }
      set({
        stats,
        player: [randomCard(), randomCard()],
        dealer: randomCard(),
      });
    },
  };
});
