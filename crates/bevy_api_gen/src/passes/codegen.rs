use crate::{configure_tera, Args, BevyCtxt, TemplateKind};

use log::info;
use rustc_hir::def_id::LOCAL_CRATE;
use std::{
    fs::{self, File},
    str::FromStr,
};
use tera::Context;

/// generates a module with the appropriate wrappers for all the found reflection ADT's in the crate
pub(crate) fn codegen(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    let (output, templates, template_args) = match &args.cmd {
        crate::Command::Generate {
            output,
            templates,
            template_args,
            ..
        } => (output, templates, template_args),
        _ => return true,
    };

    let tera = configure_tera(ctxt.tcx.crate_name(LOCAL_CRATE).as_str(), templates);

    // perform code gen using templates
    fs::create_dir_all(output).unwrap();
    info!("Writing code files to : {}", output);

    let template_data = ctxt.template_context.as_ref().unwrap();
    let mut context = Context::from_serialize(template_data).unwrap();
    if let Some(args) = template_args {
        let json = serde_json::Value::from_str(args).unwrap();
        let map = serde_json::Map::from_iter(vec![("args".to_owned(), json)]);
        let additional_ctxt = Context::from_serialize(serde_json::Value::Object(map)).unwrap();
        context.extend(additional_ctxt);
    }
    // generate crate artifact
    let file = File::create(output.join(format!("{}.rs", template_data.crate_name))).unwrap();

    tera.render_to(&TemplateKind::CrateArtifact.to_string(), &context, file)
        .expect("Failed to render crate artifact");

    true
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_templates_exist() {
        let template_files: HashSet<&str> = TEMPLATE_DIR
            .files()
            .map(|file| file.path().file_name().unwrap().to_str().unwrap())
            .collect();

        TemplateKind::VARIANTS.iter().for_each(|f| {
            assert!(
                template_files.contains(f),
                "Template file not in variants: {}",
                f
            );
        });
    }
}
