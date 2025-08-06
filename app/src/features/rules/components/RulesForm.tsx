import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  FormSubmitButton,
} from "@/components/Form";
import { Input } from "@/components/Input";
import { Switch } from "@/components/Switch";
import { zodResolver } from "@hookform/resolvers/zod";
import { FormProvider, useForm, type Resolver } from "react-hook-form";
import z from "zod";
import { useRulesState } from "../hooks/useRulesState";
import { Soft17RuleSelect } from "./Soft17RuleSelect";
import { BlackjackPayoutSelect } from "./BlackjackPayoutSelect";

export const rulesFormSchema = z.object({
  blackjackPayout: z.enum(["Ratio3to2", "Ratio6to5"]),
  numDecks: z.coerce.number().min(1).max(255),
  dealerSoft17: z.enum(["Hit", "Stand"]),
  maxSplits: z.coerce.number().min(1).max(255),
  doubleAfterSplitAllowed: z.boolean(),
  surrender: z.enum(["None", "Early", "Late"]),
});

export type RulesFormValues = z.infer<typeof rulesFormSchema>;
const resolver = zodResolver(rulesFormSchema);

export const RulesForm = () => {
  const { rules, setRules } = useRulesState();
  const form = useForm<RulesFormValues>({
    defaultValues: rules,
    resolver: resolver as Resolver<RulesFormValues>,
  });

  const onSubmit = (values: RulesFormValues) => {
    setRules(values);
    form.reset(values, { keepDirty: false });
  };

  return (
    <FormProvider {...form}>
      <form
        className="grow flex flex-col gap-4"
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <FormControl
          control={form.control}
          name="blackjackPayout"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Blackjack payout</FormLabel>
              <FormField>
                <BlackjackPayoutSelect {...field} />
              </FormField>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormControl
          control={form.control}
          name="dealerSoft17"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Dealer action on soft 17</FormLabel>
              <FormField>
                <Soft17RuleSelect {...field} />
              </FormField>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormControl
          control={form.control}
          name="numDecks"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Decks</FormLabel>
              <FormField>
                <Input {...field} type="number" />
              </FormField>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormControl
          control={form.control}
          name="maxSplits"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Max splits</FormLabel>
              <FormField>
                <Input {...field} type="number" />
              </FormField>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormControl
          control={form.control}
          name="doubleAfterSplitAllowed"
          render={({ field }) => (
            <FormItem>
              <div className="flex flex-row gap-2 items-center">
                <FormField>
                  <Switch {...field} />
                </FormField>
                <FormLabel>Double after split</FormLabel>
              </div>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormSubmitButton className="mt-auto">Save</FormSubmitButton>
      </form>
    </FormProvider>
  );
};
