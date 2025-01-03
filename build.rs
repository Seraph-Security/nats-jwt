use std::fs;

use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // if running on doc_rs the fs wis read only so we won't be able to updatte the schema
        return;
    }

    let content = std::fs::read_to_string("src/nats_jwt_schema.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let mut contents = String::from(
        r#"#![allow(unknown_lints)]
#![allow(clippy::all)]

    "#,
    );

    contents.push_str(&prettyplease::unparse(
        &syn::parse2::<syn::File>(type_space.to_stream()).unwrap(),
    ));

    fs::write("src/nats_jwt_schema.rs", contents).unwrap();

    // run "cargo fmt"
    std::process::Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("failed to run cargo fmt");
}
