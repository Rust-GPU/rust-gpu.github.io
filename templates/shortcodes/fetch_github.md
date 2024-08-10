{% if not repo %}{% set repo = config.extra.github_repo %}{% endif -%}
{% if not branch %}{% set branch = "master" %}{% endif -%}
{{ load_data(url="https://raw.githubusercontent.com/" ~ repo ~ "/" ~ branch ~ "/" ~ path) -}}
