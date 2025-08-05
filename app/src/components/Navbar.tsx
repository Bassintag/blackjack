import type { ComponentPropsWithoutRef } from "react";
import { Card } from "./Card";
import { cn } from "@/utlils/cn";
import { NavLink } from "react-router";

export const Navbar = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<typeof Card>) => {
  return <Card className={cn("py-1 px-3", className)} {...rest} />;
};

export const NavbarLinks = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<"nav">) => {
  return <nav className={cn("flex flex-row gap-3", className)} {...rest} />;
};

export const NavbarLink = ({
  className,
  ...rest
}: ComponentPropsWithoutRef<typeof NavLink>) => {
  return (
    <NavLink
      className={cn(
        "flex flex-row gap-1.5 items-center px-3 py-1.5 rounded font-semibold hover:bg-accent [&>svg]:size-5",
        className,
      )}
      {...rest}
    />
  );
};
