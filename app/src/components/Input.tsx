import { cn } from "@/utlils/cn";
import {
  forwardRef,
  type ComponentPropsWithoutRef,
  type ComponentRef,
} from "react";

export const Input = forwardRef<
  ComponentRef<"input">,
  ComponentPropsWithoutRef<"input">
>(({ className, ...rest }, ref) => {
  return (
    <input
      ref={ref}
      className={cn(
        "w-full h-9 px-2 text-sm bg-input border border-border rounded-lg outline-none focus-visible:ring-2 focus-visible:ring-primary/50",
        className,
      )}
      {...rest}
    />
  );
});
