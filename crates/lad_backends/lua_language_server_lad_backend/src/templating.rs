use include_dir::{include_dir, Dir};
use std::error::Error;

pub const TEMPLATE_DIRECTORY: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn prepare_tera() -> Result<tera::Tera, anyhow::Error> {
    let mut tera = tera::Tera::default();
    // Add the template directory to Tera
    for file in TEMPLATE_DIRECTORY.files() {
        let content_utf8 = file.contents_utf8().ok_or_else(|| {
            anyhow::anyhow!("Failed to read template file: {}", file.path().display())
        })?;

        let template_name = file.path().to_string_lossy();
        tera.add_raw_template(&template_name, content_utf8)
            .map_err(handle_tera_error)?;
        log::info!("Added template: {template_name}");
    }

    Ok(tera)
}

fn handle_tera_error(error: tera::Error) -> anyhow::Error {
    let inner_error_str = error
        .source()
        .and_then(|e| e.to_string().into())
        .unwrap_or_else(|| "No source available".to_string());
    anyhow::anyhow!("Tera error: {error}, {inner_error_str}")
}

pub fn render_template(
    template_name: &str,
    context: &tera::Context,
) -> Result<String, anyhow::Error> {
    let tera = prepare_tera()?;
    tera.get_template_names().for_each(|name| {
        log::info!("Available template: {name}");
    });
    tera.render(template_name, context)
        .map_err(handle_tera_error)
}
