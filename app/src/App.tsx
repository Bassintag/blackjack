import { Outlet } from "react-router";
import { Sidebar } from "./components/Sidebar";
import { RulesForm } from "./features/rules/components/RulesForm";
import { Navbar, NavbarLink, NavbarLinks } from "./components/Navbar";
import { ChartNoAxesCombinedIcon, SheetIcon } from "lucide-react";
import { CardContent, CardHeader, CardTitle } from "./components/Card";

const App = () => {
  return (
    <main className="min-h-dvh pl-67 flex flex-col">
      <Sidebar>
        <CardHeader>
          <CardTitle className="text-lg font-semibold">Rules</CardTitle>
        </CardHeader>
        <CardContent className="grow flex flex-col">
          <RulesForm />
        </CardContent>
      </Sidebar>
      <header className="px-3 pt-3">
        <Navbar>
          <NavbarLinks>
            <NavbarLink to="/trainer">
              <ChartNoAxesCombinedIcon />
              <span>Train</span>
            </NavbarLink>
            <NavbarLink to="/tables">
              <SheetIcon />
              <span>Strategy</span>
            </NavbarLink>
          </NavbarLinks>
        </Navbar>
      </header>
      <Outlet />
    </main>
  );
};

export const element = <App />;
