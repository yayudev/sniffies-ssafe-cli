{#- Variables -#}
{%- set feature_model = (feature_name | pascal) -%}
{%- set feature_model_plural = feature_model ~ "s" -%}
{%- set feature_model_variable = (feature_model_plural | camel) -%}
{%- set feature_slug = (feature_model_plural | kebab) -%}
{%- set feature_store = feature_model_plural ~ "Store" -%}
{%- set feature_service = feature_model_plural ~ "Service" -%}
{#- Template -#}
import { Injectable, inject } from '@angular/core';
import { {{ feature_model }} } from '{{ feature_slug}}.model';
import { {{ feature_store }} } from './{{ feature_slug }}.store';

@Injectable({ providedIn: 'root' })
export class {{ feature_service }} {
  protected readonly store = inject({{ feature_store }});

  public set{{ feature_model_plural }}({{ feature_model_variable }}: {{ feature_model }}[]) {
    this.store.set{{ feature_model_plural }}({{ feature_model_variable }});
  }

  public update{{ feature_model_plural }}({{ feature_model_variable }}: {{ feature_model }}[]) {
    this.store.update{{ feature_model_plural }}({{ feature_model_variable }});
  }
}
