export const timeout = (time: number): Promise<void> => {
  return new Promise((r) => {
    setTimeout(r, time);
  });
};
