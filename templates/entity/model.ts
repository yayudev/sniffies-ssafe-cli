{#- Variables -#}
{%- set feature_type_name = (feature_name | pascal) -%}
{#- Template -#}
export type {{ feature_type_name }} = {
  {% for property, type in properties %}
    {{ property }}: {{ type }};
  {% endfor %}
};

