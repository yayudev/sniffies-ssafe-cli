import { signalStore, withMethods, withState } from '@ngrx/signals';
import { updateState, withDevtools } from '~/core/configs/dev-tools';

import { {{ feature_name | pascal }}State } from './{{ feature_name | kebab }}.model';

const initialState: {{ feature_name | pascal }}State = {
  {% for property, value in initial_state %}{{ property }}: {{ value }},
  {% endfor %}
}

export type {{ feature_name | pascal }}Store = InstanceType<typeof {{ feature_name | pascal }}Store>;
export const {{ feature_name | pascal }}Store = signalStore(
  { providedIn: 'root' },

  withState(initialState),

  withMethods((store) => ({
    {% for property, type in properties %}
    set{{ property | pascal }}({{ property }}: {{ type }}) {
      updateState(store, "[{{ feature_name | kebab }}] set{{ property | pascal }}", () => ({ {{ property }} }));
    },
    {% endfor %}
  })),

  withDevtools('{{ feature_name | kebab }}'),
);

