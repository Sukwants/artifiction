// generated file, do not edit

export default {
    {% for c in global_configs %}
    {{ c.name }}: {
        name: "{{ c.name }}",
        config: [
            {{ c.config|e("none") }},
        ],
    },
    {% endfor %}
}
