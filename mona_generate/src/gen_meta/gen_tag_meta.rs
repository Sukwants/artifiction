use askama::Template;
use mona::character::team_status::CharacterTag;
use crate::gen_meta::gen_locale::get_index_mapping;

struct TagMeta {
    name: String,
    name_locale: usize,
}

#[derive(Template)]
#[template(path = "tag_meta_template.js")]
struct TagMetaTemplate {
    tags: Vec<TagMeta>,
}

pub fn gen_tag_meta_as_js_file() -> String {
    let mut data: Vec<TagMeta> = Vec::new();
    let index_mapping = get_index_mapping();

    for i in 0_usize..CharacterTag::LEN {
        let tag_enum: CharacterTag = num::FromPrimitive::from_usize(i).unwrap();
        data.push(TagMeta {
            name: tag_enum.to_string(),
            name_locale: *index_mapping.get(&tag_enum.get_locale()).unwrap(),
        })
    }

    let template = TagMetaTemplate {
        tags: data,
    };

    template.render().unwrap()
}