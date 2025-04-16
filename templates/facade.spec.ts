import { TestBed } from "@angular/core/testing";
import { {{ feature_name | pascal }}Facade } from "./{{ feature_name | kebab }}.facade";
import { {{ feature_name | pascal }}Store } from "./{{ feature_name | kebab }}.store";

describe("{{ feature_name | pascal }}Facade", () => {
  let store: {{ feature_name | pascal }}Facade;
  let facade: {{ feature_name | pascal }}Store;

  beforeEach(() => {
    store = TestBed.inject({{ feature_name | pascal }}Store);
    facade = TestBed.inject({{ feature_name | pascal }}Facade);
  });

  // Your tests go here
});
