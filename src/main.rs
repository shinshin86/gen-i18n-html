extern crate handlebars;
extern crate serde;
extern crate serde_json;

use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <template_path> <input_json_directory> <output_html_directory>", args[0]);
        std::process::exit(1);
    }

    let template_path = &args[1];
    let input_json_directory = &args[2];
    let output_html_directory = &args[3];

    let handlebars = Handlebars::new();

    let template = fs::read_to_string(template_path).unwrap();

    for entry in fs::read_dir(input_json_directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let lang = path.file_stem().unwrap().to_str().unwrap();

            let lang_data: BTreeMap<String, String> = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            
            let rendered_html = handlebars.render_template(&template, &lang_data).unwrap();

            let output_path = Path::new(output_html_directory).join(format!("{}.html", lang));
            fs::write(output_path, rendered_html).expect("Unable to write to output HTML file");
        }
    }
}
