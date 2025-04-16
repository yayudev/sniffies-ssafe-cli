import { Injectable, inject } from "@angular/core";
import { {{ feature_name | pascal }}Store } from "./{{ feature_name | kebab }}.store.ts";

@Injectable({
  providedIn: "root",
})
export class {{ feature_name | pascal }}Facade {
  protected readonly store = inject({{ feature_name | pascal }}Store);

  /**************
   * Properties *
   **************/

  // Your factories and computeds go here.

  /***********
   * Methods *
   ***********/

   // Your private methods go here
}
