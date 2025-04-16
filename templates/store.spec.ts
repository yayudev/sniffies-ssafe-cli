import { TestBed } from '@angular/core/testing';
import { {{ feature_name | pascal }}Store } from './{{ feature_name | kebab }}.store';

{% for property, type in properties %}
const INPUT_FOR_{{ property | constant }}: {{ type }} = /* Add input here */;
{% endfor %}

describe('{{ feature_name | pascal }}Store', () => {
  let store: {{ feature_name | pascal }}Store;

  beforeEach(() => {
    store = TestBed.inject({{ feature_name | pascal }}Store);
  });

  describe('methods', () => {
    {% for property, type in properties %}
    describe('set{{ property | pascal }}', () => {
      it ('should update the {{ property | pascal }}', () => {
        let input = INPUT_FOR_{{ property | constant }};

        store.set{{ property | pascal }}(input);
        expect(store.{{ property }}()).toBe(input);
      });
    });
    {% endfor %}
  });
});
