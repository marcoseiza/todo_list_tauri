export const onEnter = (
  input: HTMLInputElement | HTMLTextAreaElement,
  f: (this: HTMLInputElement | HTMLTextAreaElement, e: KeyboardEvent) => void
) => {
  function handleEnterListener(e: KeyboardEvent) {
    if (e.key == "Enter") f.bind(this)(e);
  }

  input.addEventListener("keypress", handleEnterListener);
  return {
    destroy() {
      input.removeEventListener("keypress", handleEnterListener);
    },
  };
};
