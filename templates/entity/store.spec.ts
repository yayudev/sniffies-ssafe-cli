{#- Variables -#}
{%- set feature_model = (feature_name | pascal) -%}
{%- set feature_model_plural  = feature_model ~ "s" -%}
{%- set feature_store = feature_model_plural ~ "Store" -%}
{%- set feature_factory = feature_model_plural ~ "Factory" -%}
{%- set feature_slug = (feature_model_plural | kebab) -%}
{%- set feature_model_variable = (feature_model_plural | camel) -%}
{#- Template -#}
import { TestBed } from '@angular/core/testing';

import { {{ feature_store  }} } from './{{ feature_slug }}.store';
import { {{ feature_factory }} } from './{{ feature_slug }}.factory';

describe('{{ feature_store }}', () => {
  let store: {{ feature_store }};

  beforeEach(() => {
    store = TestBed.inject({{ feature_store }});
  });

  describe('methods', () => {
    describe('set{{ feature_model_plural }}', () => {
      it('should set the {{ feature_model_plural }}', () => {
        // Arrange
        const {{ feature_model_variable  }} = [
          {{ feature_factory }}.create{{ feature_model }}({ id: 'test-1' }),
          {{ feature_factory }}.create{{ feature_model }}({ id: 'test-2' }),
        ];

        // Act
        store.set{{ feature_model_plural }}({{ feature_model_variable }});
      
        // Assert
        expect(store.ids()).toHaveLength(2);
        expect(store.entityMap()['test-1']).toEqual({{ feature_model_variable }}[0]);
        expect(store.entityMap()['test-2']).toEqual({{ feature_model_variable }}[1]);
      });
    });

    describe('update{{ feature_model_plural }}', () => {
      it('should update the {{ feature_model_plural }}', () => {
        // Arrange
        const {{ feature_model_variable  }} = [
          {{ feature_factory }}.create{{ feature_model }}({ id: 'test-1' }),
          {{ feature_factory }}.create{{ feature_model }}({ id: 'test-2' }),
        ];
        store.set{{ feature_model_plural }}({{ feature_model_variable }});
        
        // Act
        throw new Error("TODO: Implement update logic"));
        store.update{{ feature_model_plural }}([]);

      
        // Assert
        expect(store.ids()).toHaveLength(2);
        expect(store.entityMap()['test-1']).toEqual({{ feature_model_variable }}[0]);
        expect(store.entityMap()['test-2']).toEqual({{ feature_model_variable }}[1]);
      });
    });
  });
});
