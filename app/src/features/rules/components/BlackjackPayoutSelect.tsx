import {
  Select,
  SelectItem,
  SelectContent,
  SelectTrigger,
} from "@/components/Select";
import type { BlackjackPayout } from "@blackjack/wasm";
import { forwardRef, type ComponentRef } from "react";

export interface BlackjackPayoutSelectProps {
  value?: BlackjackPayout;
  onChange?: (value: BlackjackPayout) => void;
}

export const BlackjackPayoutSelect = forwardRef<
  ComponentRef<typeof SelectTrigger>,
  BlackjackPayoutSelectProps
>(({ value, onChange }, ref) => {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger ref={ref} />
      <SelectContent>
        <SelectItem value="Ratio3to2">3 to 2</SelectItem>
        <SelectItem value="Ratio6to5">6 to 5</SelectItem>
      </SelectContent>
    </Select>
  );
});
