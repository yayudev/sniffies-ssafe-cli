import { TestBed } from "@angular/core/testing";
import { CruiseThisAreaFacade } from "./cruise-this-area.facade";
import { CruiseThisAreaStore } from "./cruise-this-area.store";

describe("CruiseThisAreaFacade", () => {
  let store: CruiseThisAreaFacade;
  let facade: CruiseThisAreaStore;

  beforeEach(() => {
    store = TestBed.inject(CruiseThisAreaStore);
    facade = TestBed.inject(CruiseThisAreaFacade);
  });

  // Your tests go here
});
