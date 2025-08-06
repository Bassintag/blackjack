import { cn } from "@/utlils/cn";
import * as S from "@radix-ui/react-select";
import { CheckIcon, ChevronDownIcon } from "lucide-react";
import {
  forwardRef,
  type ComponentPropsWithoutRef,
  type ComponentRef,
} from "react";

export const Select = S.Root;

export const SelectTrigger = forwardRef<
  ComponentRef<typeof S.Trigger>,
  ComponentPropsWithoutRef<typeof S.Trigger>
>(({ className, ...rest }, ref) => {
  return (
    <S.Trigger
      ref={ref}
      className={cn(
        "w-full h-9 px-2 flex flex-row gap-2 items-center text-sm bg-input border border-border rounded-lg outline-none",
        className,
      )}
      {...rest}
    >
      <div className="grow text-start">
        <S.SelectValue />
      </div>
      <S.SelectIcon>
        <ChevronDownIcon className="size-4" />
      </S.SelectIcon>
    </S.Trigger>
  );
});

export const SelectContent = ({
  className,
  children,
  position = "popper",
  ...rest
}: ComponentPropsWithoutRef<typeof S.Content>) => {
  return (
    <S.Portal>
      <S.Content
        className={cn(
          "relative bg-input text-accent-foreground border border-border overflow-x-auto overflow-y-auto rounded-md shadow-md z-50 min-w-[8rem] max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin)",
          "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1",
          className,
        )}
        position={position}
        {...rest}
      >
        <S.Viewport className="h-[var(--radix-select-trigger-height)] w-full min-w-[var(--radix-select-trigger-width)] scroll-my-1">
          {children}
        </S.Viewport>
      </S.Content>
    </S.Portal>
  );
};

export const SelectItem = ({
  className,
  children,
  ...rest
}: ComponentPropsWithoutRef<typeof S.Item>) => {
  return (
    <S.Item
      className={cn(
        "flex flex-row items-center py-1.5 pr-8 pl-2 text-sm select-none outline-hidden hover:bg-accent hover:text-accent-foreground",
        className,
      )}
      {...rest}
    >
      <span className="absolute right-2 flex size-3.5 items-center justify-center">
        <S.ItemIndicator>
          <CheckIcon className="size-4" />
        </S.ItemIndicator>
      </span>
      <S.ItemText>{children}</S.ItemText>
    </S.Item>
  );
};
