export type Color = keyof typeof colors;

const colors = {
  yellow: "text-amber-500",
  red: "text-red-500",
  green: "text-green-500",
  blue: "text-cyan-500",
};

export const textColor = (c: Color) => {
  return colors[c];
};

const backgroundColors = {
  yellow: "bg-amber-500",
  red: "bg-red-500",
  green: "bg-green-500",
  blue: "bg-blue-500",
};

export const backgroundColor = (c: Color) => {
  return backgroundColors[c];
};
