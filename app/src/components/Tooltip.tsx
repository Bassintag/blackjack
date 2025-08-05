import { cn } from "@/utlils/cn";
import * as T from "@radix-ui/react-tooltip";
import type { ComponentPropsWithoutRef } from "react";

export const Tooltip = (props: ComponentPropsWithoutRef<typeof T.Root>) => {
  return (
    <T.Provider>
      <T.Root {...props} />
    </T.Provider>
  );
};

export const TooltipTrigger = T.Trigger;

export const TooltipContent = ({
  className,
  children,
  ...rest
}: ComponentPropsWithoutRef<typeof T.Content>) => {
  return (
    <T.Portal>
      <T.Content
        className={cn(
          "bg-background text-foreground px-3 py-1.5 rounded shadow z-50 w-fit text-xs m-1.5 origin-(--radix-tooltip-content-transform-origin)",
          className,
        )}
        {...rest}
      >
        {children}
      </T.Content>
    </T.Portal>
  );
};
