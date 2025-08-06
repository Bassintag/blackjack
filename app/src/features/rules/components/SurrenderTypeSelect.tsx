import {
  Select,
  SelectItem,
  SelectContent,
  SelectTrigger,
} from "@/components/Select";
import type { SurrenderType } from "@blackjack/wasm";
import { forwardRef, type ComponentRef } from "react";

export interface SurrenderTypeSelectProps {
  value?: SurrenderType;
  onChange?: (value: SurrenderType) => void;
}

export const SurrenderTypeSelect = forwardRef<
  ComponentRef<typeof SelectTrigger>,
  SurrenderTypeSelectProps
>(({ value, onChange }, ref) => {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger ref={ref} />
      <SelectContent>
        <SelectItem value="None">None</SelectItem>
        <SelectItem value="Early">Early</SelectItem>
        <SelectItem value="Late">Late</SelectItem>
      </SelectContent>
    </Select>
  );
});
