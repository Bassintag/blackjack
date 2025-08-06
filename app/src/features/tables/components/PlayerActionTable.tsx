import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/Tooltip";
import { cn } from "@/utlils/cn";
import { backgroundColor } from "@/utlils/color";
import { getPlayerActionConfig } from "@/utlils/playerAction";
import type { StrategyTable } from "@blackjack/wasm";
import React, { useMemo, type ComponentPropsWithoutRef } from "react";

export type TableType = "hard" | "soft" | "pair";

interface TableTypeConfig {
  title: string;
  format: (v: number) => string;
}

const format = (v: number) => (v === 11 ? "A" : v.toFixed(0));

const configs: Record<TableType, TableTypeConfig> = {
  hard: {
    title: "Hard",
    format: (v) => v.toFixed(),
  },
  soft: {
    title: "Soft",
    format: (v) => `A ${v - 11}`,
  },
  pair: {
    title: "Pair",
    format: (v) => `${v} ${v}`,
  },
};

export interface PlayerActionTableProps {
  table: StrategyTable;
  type: TableType;
}

export const PlayerActionTable = ({ table, type }: PlayerActionTableProps) => {
  const { headerRow, rows } = useMemo(() => {
    const from = 2;
    const to = 11;
    const cells: React.ReactNode[] = [];
    const typeConfig = configs[type];
    for (let x = from; x <= to; x += 1) {
      cells.push(
        <td className="py-1" key={x}>
          {format(x)}
        </td>,
      );
    }
    const headerRow = (
      <tr>
        <td className="bg-accent" />
        {cells}
      </tr>
    );

    const rows: React.ReactNode[] = [];
    const height = table.to - table.from;
    for (let y = 0; y <= height; y += 1) {
      const cells: React.ReactNode[] = [];
      const width = to - from + 1;
      for (let x = 0; x < width; x += 1) {
        const value = table.values[x + y * width];
        const config = getPlayerActionConfig(value.action);
        cells.push(
          <Tooltip key={x} delayDuration={0} disableHoverableContent>
            <TooltipTrigger
              asChild
              className={cn(
                "font-semibold text-sm px-3 py-1 text-center text-background hover:bg-background hover:text-foreground",
                backgroundColor(config.color),
              )}
            >
              <td>{config.short}</td>
            </TooltipTrigger>
            <TooltipContent>
              <ul className="flex flex-col">
                <InfoRow>
                  <InfoTitle>EV</InfoTitle>
                  <InfoValue>{value.ev}</InfoValue>
                </InfoRow>
                <InfoRow>
                  <InfoTitle>Hit</InfoTitle>
                  <InfoValue>{value.evs.hit}</InfoValue>
                </InfoRow>
                <InfoRow>
                  <InfoTitle>Stand</InfoTitle>
                  <InfoValue>{value.evs.stand}</InfoValue>
                </InfoRow>
                <InfoRow>
                  <InfoTitle>Double</InfoTitle>
                  <InfoValue>{value.evs.double}</InfoValue>
                </InfoRow>
                {value.evs.split != null && (
                  <InfoRow>
                    <InfoTitle>Split</InfoTitle>
                    <InfoValue>{value.evs.split}</InfoValue>
                  </InfoRow>
                )}
              </ul>
            </TooltipContent>
          </Tooltip>,
        );
      }
      rows.push(
        <tr key={y}>
          <td className="px-3">{typeConfig.format(y + table.from)}</td>
          {cells}
        </tr>,
      );
    }
    return { headerRow, rows };
  }, [table, type]);

  return (
    <div>
      <h2 className="text-center text-xl font-bold mb-3">
        {configs[type].title}
      </h2>
      <table className="text-center bg-background text-foreground rounded overflow-hidden">
        <thead>{headerRow}</thead>
        <tbody>{rows}</tbody>
      </table>
    </div>
  );
};

const InfoRow = ({ className, ...rest }: ComponentPropsWithoutRef<"li">) => {
  return (
    <li
      className={cn("flex flex-row items-center gap-2", className)}
      {...rest}
    />
  );
};

const InfoTitle = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"span">) => {
  return <span className={cn("mr-auto font-semibold", className)} {...rest} />;
};

const InfoValue = ({
  className,
  children,
  ...rest
}: Omit<ComponentPropsWithoutRef<"span">, "children"> & {
  children: number;
}) => {
  return (
    <span
      className={cn(
        "font-bold",
        { "text-green-500": children >= 0, "text-red-500": children < 0 },
        className,
      )}
      {...rest}
    >
      {children.toFixed(2)}
    </span>
  );
};
