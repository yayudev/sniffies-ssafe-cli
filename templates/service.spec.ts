import { {{ feature_name | pascal }}Service } from "./{{ feature_name | kebab }}.service";
import { {{ feature_name | pascal }}Store } from "./{{ feature_name | kebab }}.store";

{% for property, type in properties %}
const INPUT_FOR_{{ property | constant }}: {{ type }} = /* PUT_YOUR_TEST_INPUT_HERE */ ;
{% endfor %}

describe("{{ feature_name | pascal | safe }}Service", () => {
  let store: {{ feature_name | pascal }}Store;
  let service: {{ feature_name | pascal }}Service;

  beforeEach(() => {
    store = inject({{ feature_name | pascal }}Store);
    service = inject({{ feature_name | pascal }}Service);
  });

  {% for property, type in properties %}
    describe('set{{ property | pascal }}', () => {
      it('should set {{ property }} when called', () => {
        const input = INPUT_FOR_{{ property | constant }};
        service.set{{ property | pascal }}(input);

        expect(store.{{ property }}()).toBe(input);
      })
    });
  {% endfor %}
});
