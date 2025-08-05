import { cn } from "@/utlils/cn";
import { Slot } from "@radix-ui/react-slot";
import {
  createContext,
  useContext,
  useId,
  type ComponentPropsWithoutRef,
  type PropsWithChildren,
} from "react";
import {
  Controller,
  useFormContext,
  useFormState,
  type ControllerProps,
  type FieldPath,
  type FieldValues,
} from "react-hook-form";
import { Button, type ButtonProps } from "./Button";

interface FormControlContextValue {
  name: string;
}

const FormControlContext = createContext<FormControlContextValue | null>(null);

const useFormControlContext = () =>
  useContext(FormControlContext) as FormControlContextValue;

export interface FormControlProps<
  TFieldValues extends FieldValues = FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
  TTransformedValues = TFieldValues,
> extends ControllerProps<TFieldValues, TName, TTransformedValues> {}

export const FormControl = <
  TFieldValues extends FieldValues = FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
  TTransformedValues = TFieldValues,
>(
  props: FormControlProps<TFieldValues, TName, TTransformedValues>,
) => {
  return (
    <FormControlContext.Provider value={{ name: props.name }}>
      <Controller {...props} />
    </FormControlContext.Provider>
  );
};

export interface FormItemContextValue {
  id: string;
}

export const FormItemContext = createContext<FormItemContextValue | null>(null);

export const useFormFieldContext = () =>
  useContext(FormItemContext) as FormItemContextValue;

export const FormItem = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"div">) => {
  const id = useId();
  return (
    <FormItemContext.Provider value={{ id }}>
      <div className={cn("grid gap-2", className)} {...rest} />
    </FormItemContext.Provider>
  );
};

export const FormLabel = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"label">) => {
  const { id } = useFormFieldContext();
  return (
    <label
      className={cn("text-sm font-medium text-muted-foreground", className)}
      htmlFor={id}
      {...rest}
    />
  );
};

export const FormField = ({ children }: PropsWithChildren) => {
  const { id } = useFormFieldContext();
  return <Slot id={id}>{children}</Slot>;
};

export const FormMessage = ({
  className,
  children,
  ...rest
}: ComponentPropsWithoutRef<"p">) => {
  const { name } = useFormControlContext();
  const { getFieldState } = useFormContext();
  const formState = useFormState({ name });
  const { error } = getFieldState(name, formState);
  const content = error ? String(error?.message ?? "") : children;

  if (!content) {
    return null;
  }
  return (
    <p className={cn("text-xs text-red-500", className)} {...rest}>
      {content}
    </p>
  );
};

export const FormSubmitButton = (props: ButtonProps) => {
  const { isDirty, isSubmitting } = useFormState();
  return (
    <Button type="submit" disabled={!isDirty || isSubmitting} {...props} />
  );
};
