import { Injectable, inject } from "@angular/core";
import { {{ feature_name | pascal }}Store } from "./{{ feature_name | kebab }}.store";


@Injectable({ providedIn: "root" })
export class {{ feature_name | pascal }}Service {
  protected readonly store = inject({{ feature_name | pascal }}Store);

  {% for property, type in properties %}
  set{{ property | pascal }}({{ property }}: {{ type }}) {
    this.store.set{{ property | pascal }}({{ property }});
  }
  {% endfor %}
}
