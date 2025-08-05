import type { Rank } from "@blackjack/wasm";
import { CardRank, type CardValue } from "../domain/CardValue";

export const asRank = (card: CardValue): Rank => {
  switch (card.rank) {
    case CardRank.ACE:
      return "Ace";
    case CardRank.TWO:
      return "Two";
    case CardRank.THREE:
      return "Three";
    case CardRank.FOUR:
      return "Four";
    case CardRank.FIVE:
      return "Five";
    case CardRank.SIX:
      return "Six";
    case CardRank.SEVEN:
      return "Seven";
    case CardRank.EIGHT:
      return "Eight";
    case CardRank.NINE:
      return "Nine";
    case CardRank.TEN:
      return "Ten";
    case CardRank.JACK:
      return "Jack";
    case CardRank.QUEEN:
      return "Queen";
    case CardRank.KING:
      return "King";
  }
};
