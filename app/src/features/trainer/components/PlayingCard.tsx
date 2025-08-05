import type { ComponentPropsWithoutRef } from "react";
import spritesheet from "@/assets/CuteCards.png";
import { cn } from "@/utlils/cn";
import type { CardValue } from "@/domain/CardValue";

const WIDTH = 1500;
const HEIGHT = 596;

const SPRITES_X = 15;
const SPRITES_Y = 4;

const SPRITE_W = WIDTH / SPRITES_X;
const SPRITE_H = HEIGHT / SPRITES_Y;

export interface PlayingCardProps extends ComponentPropsWithoutRef<"div"> {
  value: CardValue;
}

export const PlayingCard = ({
  value: { rank, color },
  className,
  ...rest
}: PlayingCardProps) => {
  const x = -(rank * SPRITE_W);
  const y = -(color * SPRITE_H);

  return (
    <div
      style={{
        width: SPRITE_W,
        height: SPRITE_H,
        background: `url(${spritesheet})`,
        backgroundPositionX: x,
        backgroundPositionY: y,
        backgroundSize: `${WIDTH}px ${HEIGHT}px`,
      }}
      className={cn("inline-block hover:scale-110 transition", className)}
      {...rest}
    />
  );
};
