{% set components = item | split(pat=".") -%}
{% if not crate %}{% set crate = config.extra.rust_crate %}{% endif -%}
{% if not no_link %}[`{{ components|last }}{% if type == "macro" %}!{% endif %}`]({% endif %}https://docs.rs/{{ crate }}/latest/{{ crate }}/{% if components|length > 1 %}{{ components|slice(end=-1)|join(sep="/") }}/{% endif %}{{ type }}.{{ components|last }}.html{% if not no_link %}){% endif -%}
