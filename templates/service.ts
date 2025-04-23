{#- Variables -#}
{%- set feature_model = (feature_name | pascal) -%}
{%- set feature_slug = (feature_name | kebab) -%}
{%- set feature_store = feature_name ~ "Store" -%}
{%- set feature_service = feature_name ~ "Service" -%}
{#- Template -#}
import { Injectable, inject } from "@angular/core";
import { {{ feature_store }} from "./{{ feature_slug }}.store";


@Injectable({ providedIn: "root" })
export class {{ feature_service }} {
  protected readonly store = inject({{ feature_store }});

  {%- for property, type in properties -%}
  set{{ property | pascal }}({{ property }}: {{ type }}) {
    this.store.set{{ property | pascal }}({{ property }});
  }
  {%- endfor -%}
}
