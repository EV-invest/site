"use client";

import { createContext, useContext, useEffect, type ReactNode } from "react";
import { capture } from "@/features/analytics";
import type { ExperimentKey } from "@/shared/config/experiments";

type ExperimentCtx = { experiment: ExperimentKey; variant: string };

const ExperimentContext = createContext<ExperimentCtx | null>(null);

/**
 * Client island that marks a section's A/B boundary.
 *
 * Fires `${experiment}_exposed` once on mount so PostHog records which variant
 * was served. Provides `{ experiment, variant }` context to every
 * {@link useExperimentEvent} call inside the subtree.
 *
 * Place in the section's **server wrapper** and pass server-rendered children —
 * the children prop pattern keeps the variant content out of the client bundle.
 *
 * @example
 * ```tsx
 * export async function Hero() {
 *   const variant = await getVariant("hero");
 *   return (
 *     <ExperimentTracker experiment="hero" variant={variant}>
 *       {match(variant, { a: <HeroA />, b: <HeroB /> })}
 *     </ExperimentTracker>
 *   );
 * }
 * ```
 */
export function ExperimentTracker({
  experiment,
  variant,
  children,
}: {
  experiment: ExperimentKey;
  variant: string;
  children: ReactNode;
}) {
  useEffect(() => {
    capture(`${experiment}_exposed`, { variant });
  }, [experiment, variant]);

  return (
    <ExperimentContext.Provider value={{ experiment, variant }}>
      {children}
    </ExperimentContext.Provider>
  );
}

/**
 * Returns a `track` function scoped to the nearest {@link ExperimentTracker}.
 *
 * The emitted event name is `${experiment}_${action}`. The active `variant` is
 * merged into props automatically, so callers never thread variant down manually.
 *
 * @throws {Error} When called outside an `<ExperimentTracker>` subtree.
 *
 * @example
 * ```tsx
 * const track = useExperimentEvent();
 *
 * // Fire immediately:
 * track("cta_clicked", { cta: "explore" });
 *
 * // Fire + side effect — handler controls the order:
 * track("cta_clicked", { cta: "explore" }, (fire) => {
 *   fire();
 *   doSomethingElse();
 * });
 * ```
 */
export function useExperimentEvent() {
  const ctx = useContext(ExperimentContext);
  if (!ctx) throw new Error("useExperimentEvent must be used inside <ExperimentTracker>");

  const { experiment, variant } = ctx;

  /**
   * @param action  Event action suffix — emitted as `${experiment}_${action}`.
   * @param props   Extra primitive, non-PII props merged into the payload.
   * @param handler When provided, called with `fire` instead of firing directly.
   *                The handler decides when `fire()` runs relative to side effects.
   */
  return function track(
    action: string,
    props?: Record<string, unknown>,
    handler?: (fire: () => void) => void,
  ) {
    const fire = () => capture(`${experiment}_${action}`, { variant, ...props });
    if (handler) {
      handler(fire);
    } else {
      fire();
    }
  };
}
