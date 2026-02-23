// generated file, do not edit (not for *_template.js)
export default {
    {% for tag in tags %}
    {{ tag.name }}: {
        name: "{{ tag.name }}",
        nameLocale: {{ tag.name_locale }},
    },
    {% endfor %}
}