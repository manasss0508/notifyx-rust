use std::collections::HashMap;
use futures_util::StreamExt;
use crate::datamodels::database::template::Template;

pub fn template_render(template: Template, variables: &String) -> (String,String) {

    // converting variables string to hashmap
    let variables:HashMap<String,String> = serde_json::from_str(variables).unwrap();

    // render subject
    let subject = template_interpolate(template.subject,&variables);

    // render body
    let body = template_interpolate(template.body,&variables);

    (subject,body)
}

fn template_interpolate(mut string: String, variables: &HashMap<String,String>) -> String {
    while let Some(open) = string.find("{{") {
        let after_open = open + 2;

        let Some(relative_end) = string[after_open..].find("}}") else {
            break
        };

        let end = after_open + relative_end;

        let variable = &string[after_open..end];

        let value = variables.get(variable).unwrap();

        string.replace_range(open..end+2,value)
    }

    string
}   