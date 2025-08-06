import { cn } from "@/utlils/cn";
import type { ComponentPropsWithoutRef } from "react";
import { useTrainerState } from "../hooks/useTrainerState";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/Card";

export const TrainerStats = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<typeof Card>) => {
  const { total, failures, success } = useTrainerState((s) => s.stats);
  return (
    <Card className={cn("w-40", className)} {...rest}>
      <CardHeader>
        <CardTitle>Stats</CardTitle>
      </CardHeader>
      <CardContent>
        <div>Total: {total}</div>
        <div>Correct: {success}</div>
        <div>Errors: {failures}</div>
      </CardContent>
    </Card>
  );
};
