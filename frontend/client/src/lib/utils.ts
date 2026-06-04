import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { toast } from "sonner";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// Every interactive control in the design mock is a stub. Clicking one raises
// the same "this is a concept" toast — kept here so each section calls it
// rather than re-declaring the copy.
export function notifyPlaceholder(featureName: string) {
  toast.info(`${featureName} — Концепт-интерфейс`, {
    description: "Данный элемент является частью интерактивного дизайн-макета.",
    duration: 3000
  });
}
