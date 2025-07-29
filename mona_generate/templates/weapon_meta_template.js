// generated file, do not edit
{% for w in weapons %}
import {{ w.name }}_tn from "@image/weapons/{{ w.name }}_tn"
{% endfor %}

// {# const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/UI_EquipIcon_#.png"
// const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_u9b0pg/#.png"
// const imageUrl = name => template.replace("#", name)
// const newImageUrl = hash => newTemplate.replace("#", hash) #}

export default {
{% for w in weapons %}
    {{ w.name }}: {
        name: "{{ w.name }}",
        internalName: "{{ w.internal_name }}",
        nameLocale: {{w.name_index}},
        star: {{ w.star }},
        // {# {% if w.icon_hash == "" -%}
        // url: imageUrl("{{ w.internal_name }}"),
        // {% else -%}
        // url: newImageUrl("{{ w.icon_hash }}"),
        // {%- endif %} #}
        url: {{ w.name }}_tn,
        type: "{{ w.t }}",

        {% if w.effect.is_some() %}
        effect: {{w.effect.unwrap()}},
        {% endif %}
        {% if w.configs.len() > 0 %}
        configs: [
            {% for config in w.configs %}
            {{ config|e("none") }},
            {% endfor %}
        ],
        {% else %}
        configs: null,
        {% endif %}
    },
{% endfor %}
}
