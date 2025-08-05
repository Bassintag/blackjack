import { createBrowserRouter, Navigate } from "react-router";

export const router = createBrowserRouter([
  {
    lazy: () => import("@/App"),
    children: [
      {
        path: "trainer",
        lazy: () => import("@/features/trainer/pages/TrainerPage"),
      },
      {
        path: "tables",
        lazy: () => import("@/features/tables/pages/TablesPage"),
      },
    ],
  },
  {
    path: "*",
    element: <Navigate to="trainer" />,
  },
]);
