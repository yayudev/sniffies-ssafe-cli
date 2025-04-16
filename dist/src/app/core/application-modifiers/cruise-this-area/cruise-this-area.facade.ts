import { Injectable, inject } from "@angular/core";
import { CruiseThisAreaStore } from "./cruise-this-area.store.ts";

@Injectable({
  providedIn: "root",
})
export class CruiseThisAreaFacade {
  protected readonly store = inject(CruiseThisAreaStore);

  /**************
   * Properties *
   **************/

  // Your factories and computeds go here.

  /***********
   * Methods *
   ***********/

   // Your private methods go here
}
