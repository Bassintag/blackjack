import { useRulesState } from "@/features/rules/hooks/useRulesState";
import { StrategyGenerator } from "@blackjack/wasm";
import {
  createContext,
  useContext,
  useMemo,
  type PropsWithChildren,
} from "react";

export const StrategyGeneratorContext = createContext<StrategyGenerator | null>(
  null,
);

export const useStrategyGenerator = () =>
  useContext(StrategyGeneratorContext) as StrategyGenerator;

export const StrategyGeneratorProvider = ({ children }: PropsWithChildren) => {
  const rules = useRulesState((s) => s.rules);

  const strategyGenerator = useMemo(() => {
    return new StrategyGenerator(rules);
  }, [rules]);

  return (
    <StrategyGeneratorContext.Provider value={strategyGenerator}>
      {children}
    </StrategyGeneratorContext.Provider>
  );
};
