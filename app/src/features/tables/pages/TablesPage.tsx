import { useStrategyGenerator } from "@/contexts/StrategyGeneratorContext";
import { useMemo } from "react";
import { PlayerActionTable } from "../components/PlayerActionTable";

const TablesPage = () => {
  const strategyGenerator = useStrategyGenerator();

  const tables = useMemo(() => {
    return strategyGenerator.tables();
  }, [strategyGenerator]);

  return (
    <div className="flex flex-col items-center gap-6 py-3">
      <PlayerActionTable table={tables.hard} type="hard" />
      <PlayerActionTable table={tables.soft} type="soft" />
      <PlayerActionTable table={tables.pair} type="pair" />
    </div>
  );
};

export const element = <TablesPage />;
