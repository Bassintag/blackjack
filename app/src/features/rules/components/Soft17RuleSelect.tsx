import {
  Select,
  SelectItem,
  SelectContent,
  SelectTrigger,
} from "@/components/Select";
import type { Soft17Rule } from "@blackjack/wasm";
import { SelectIcon, SelectValue } from "@radix-ui/react-select";
import { ChevronDownIcon } from "lucide-react";
import { forwardRef, type ComponentRef } from "react";

export interface Soft17RuleSelectProps {
  value?: Soft17Rule;
  onChange?: (value: Soft17Rule) => void;
}

export const Soft17RuleSelect = forwardRef<
  ComponentRef<typeof SelectTrigger>,
  Soft17RuleSelectProps
>(({ value, onChange }, ref) => {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger
        ref={ref}
        className="w-full h-9 px-2 flex flex-row gap-2 items-center text-sm bg-input border border-border rounded-lg outline-none"
      >
        <div className="grow text-start">
          <SelectValue />
        </div>
        <SelectIcon>
          <ChevronDownIcon className="size-4" />
        </SelectIcon>
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="Hit">Hit</SelectItem>
        <SelectItem value="Stand">Stand</SelectItem>
      </SelectContent>
    </Select>
  );
});
