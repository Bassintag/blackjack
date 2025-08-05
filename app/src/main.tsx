import "@/main.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { StrategyGeneratorProvider } from "./contexts/StrategyGeneratorContext";
import { router } from "./router";
import { RouterProvider } from "react-router";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <StrategyGeneratorProvider>
      <RouterProvider router={router} />
    </StrategyGeneratorProvider>
  </StrictMode>,
);
