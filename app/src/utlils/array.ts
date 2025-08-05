export const randomPick = <T>(arr: T[]) => {
  const i = Math.floor(Math.random() * arr.length);
  return arr[i];
};
