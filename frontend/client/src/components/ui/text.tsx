import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

/**
 * Two text colors, named by role. The page text is Limestone Mist (--color-main-mist)
 * shown at two opacities:
 *   info      — primary reading copy. Bundles the recurring recipe (light, relaxed,
 *               default text-sm). Size/spacing are overridable via className.
 *   secondary — quieter supporting copy: stat labels, captions, field labels, footer.
 *               COLOR ONLY — these usages share nothing but the opacity, so casing,
 *               size and font (e.g. font-mono-tech uppercase) stay on the caller.
 */
const textVariants = cva("", {
  variants: {
    variant: {
      info: "text-sm font-light leading-relaxed text-main-mist/70",
      secondary: "text-main-mist/40",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

function Text({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<"p"> &
  VariantProps<typeof textVariants> & { asChild?: boolean }) {
  const Comp = asChild ? Slot : "p";

  return (
    <Comp
      data-slot="text"
      className={cn(textVariants({ variant }), className)}
      {...props}
    />
  );
}

export { Text, textVariants };
