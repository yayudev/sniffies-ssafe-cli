import { signalStore, withMethods, withState } from "@ngrx/signals";
import { updateState, withDevtools } from "~/core/configs/dev-tools";

import { CruiseThisAreaState } from "./cruise-this-area.model";

const initialState: CruiseThisAreaState = {
  canRender: true,
  radius: 42.069,
};

export type CruiseThisAreaStore = InstanceType<typeof CruiseThisAreaStore>;
export const CruiseThisAreaStore = signalStore(
  { providedIn: "root" },

  withState(initialState),

  withMethods((store) => ({
    setCanRender(canRender: boolean) {
      updateState(store, "[cruise-this-area] setCanRender", () => ({
        canRender,
      }));
    },

    setRadius(radius: number) {
      updateState(store, "[cruise-this-area] setRadius", () => ({ radius }));
    },
  })),

  withDevtools("cruise-this-area"),
);
