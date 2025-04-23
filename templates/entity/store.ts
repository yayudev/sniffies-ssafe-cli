{#- Variables -#}
{%- set feature_model = (feature_name | pascal) -%}
{%- set feature_model_plural  = feature_model ~ "s" -%}
{%- set feature_store = feature_model_plural ~ "Store" -%}
{%- set feature_slug = (feature_model_plural | kebab) -%}
{%- set feature_model_variable = (feature_model_plural | camel) -%}
{#- Template -#}
import { signalStore, withMethods } from '@ngrx/signals';
import { setEntitites, removeEntities, updateEntities, withEntities } from '@ngrx/signals/entities';
import { updateState, withDevtools } from '~/core/configs/dev-tools';

import { {{ feature_model }} } from './{{ feature_slug }}.model';

export type {{ feature_store }} = InstanceType<typeof {{ feature_store }}>;
export const {{ feature_store }} = signalStore(
  { providedIn: 'root' },

  withEntities<{{ feature_model }}>(),

  withMethods((store) => ({
    set{{ feature_model_plural }}({{ feature_model_variable }}: {{ feature_model }}[]) {
      updateState(store, "[{{ feature_slug }}] set{{ feature_model_plural }}", setEntitites({{ feature_model_variable }}));
    },

    update{{ feature_model_plural }}({{ feature_model_variable }}: {{ feature_model }}[]) {
      updateState(store, "[{{ feature_slug }}] update{{ feature_model_plural }}", updateEntities({{ feature_model_variable }}));
    },
  })),

  withDevtools('{{ feature_slug }}'),
);

