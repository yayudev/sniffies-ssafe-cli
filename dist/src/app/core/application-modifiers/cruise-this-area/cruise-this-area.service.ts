import { Injectable, inject } from "@angular/core";
import { CruiseThisAreaStore } from "./cruise-this-area.store";

@Injectable({ providedIn: "root" })
export class CruiseThisAreaService {
  protected readonly store = inject(CruiseThisAreaStore);

  setCanRender(canRender: boolean) {
    this.store.setCanRender(canRender);
  }

  setRadius(radius: number) {
    this.store.setRadius(radius);
  }
}
