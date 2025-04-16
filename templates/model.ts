export type {{ feature_name | pascal }}State = {
  {% for property, type in properties %}
    {{ property }}: {{ type }};
  {% endfor %}
};

