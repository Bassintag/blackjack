import { cn } from "@/utlils/cn";
import * as S from "@radix-ui/react-switch";
import {
  forwardRef,
  type ComponentPropsWithoutRef,
  type ComponentRef,
} from "react";

export interface SwitchProps
  extends Omit<ComponentPropsWithoutRef<typeof S.Root>, "value" | "onChange"> {
  value?: boolean;
  onChange?: (value: boolean) => void;
}

export const Switch = forwardRef<ComponentRef<typeof S.Root>, SwitchProps>(
  ({ value, onChange, className, ...rest }, ref) => {
    return (
      <S.Root
        ref={ref}
        className={cn(
          "relative h-7 w-12 rounded-full bg-accent shadow-inner data-[state=checked]:bg-primary transition-colors",
          className,
        )}
        checked={value}
        onCheckedChange={onChange}
        {...rest}
      >
        <S.Thumb className="absolute top-1 left-1 size-5 rounded-full bg-background data-[state=checked]:left-6 transition-all" />
      </S.Root>
    );
  },
);
