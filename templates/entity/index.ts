{%- set feature_plural = feature_name ~ "s" -%}
{%- set feature_slug = (feature_plural | kebab) -%}
export * from "./{{ feature_slug }}.model";
export * from "./{{ feature_slug }}.service";
export * from "./{{ feature_slug }}.store";
