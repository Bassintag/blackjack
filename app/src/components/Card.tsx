import { cn } from "@/utlils/cn";
import type { ComponentPropsWithoutRef } from "react";

export const Card = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={cn("bg-background text-foreground rounded shadow", className)}
      {...rest}
    />
  );
};

export const CardHeader = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"div">) => {
  return <div className={cn("px-3 pt-1.5", className)} {...rest} />;
};

export const CardTitle = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"p">) => {
  return <p className={cn("font-semibold text-lg", className)} {...rest} />;
};

export const CardContent = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"div">) => {
  return <div className={cn("px-3 py-1.5", className)} {...rest} />;
};
