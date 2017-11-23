#[macro_use]
extern crate error_chain;
extern crate handlebars;
extern crate openapi;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
mod strfix;

use error_chain::ChainedError;
use handlebars::to_json;
use strfix::*;
use std::collections::{VecDeque, BTreeSet};

error_chain!{
    foreign_links {
        Handlebars(handlebars::TemplateFileError);
        Io(std::io::Error);
        OpenApi(openapi::errors::Error);
        Render(handlebars::RenderError);
        Yaml(serde_yaml::Error);
    }
}

fn main() {
    let spec = match std::env::args().nth(2) {
        Some(x) => x,
        None => "api-specs/skeleton/config.yaml".to_owned(),
    };

    if let Err(e) = run(&spec) {
        print!("{}", e.display());
        std::process::exit(1);
    };

}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GenConfig {
    version: String,
    base_dir: std::path::PathBuf,
    out_dir: std::path::PathBuf,
}

fn run(cfgfile: &str) -> Result<()> {
    let f = std::fs::File::open(&cfgfile).chain_err(
        || "unable to open cfgfile",
    )?;
    let cfg: GenConfig = serde_yaml::from_reader(std::io::BufReader::new(f))?;
    std::fs::create_dir_all(&cfg.out_dir)?;
    std::fs::create_dir_all(&cfg.out_dir.join("types"))?;
    let spec = openapi::from_path(&cfg.base_dir.join("swagger.json"))
        .chain_err(|| "unable to parse swagger.json")?;

    let mut hs = handlebars::Handlebars::new();
    hs.register_template_file(
        "mod",
        "./hbs-templates/mod.rs.hbs",
    )?;
    hs.register_template_file(
        "get",
        "./hbs-templates/get.rs.hbs",
    )?;
    hs.register_template_file(
        "types_mod",
        "./hbs-templates/types_mod.rs.hbs",
    )?;

    let mut root_map = serde_json::Map::new();
    root_map.insert("version".to_string(), to_json(&cfg.version));
    root_map.insert(
        "uversion".to_string(),
        to_json(&dot_under(cfg.version.clone())),
    );
    root_map.insert("nversion".to_string(), to_json(&nodot(cfg.version.clone())));

    let mut ops_tags = BTreeSet::<String>::new();
    ops_tags.insert("generic".to_string());

    let mut ops = serde_json::Map::new();
    for (endpoint, oa_ops) in spec.paths {
        let mut o = serde_json::Map::new();
        o.insert("endpoint".to_string(), to_json(&endpoint));
        if let Some(op) = oa_ops.get {
            if op.operation_id.is_none() {
                continue;
            }
            let fname = fix_name(op.operation_id.unwrap());
            o.insert("fname".to_string(), to_json(&fname));
            if let Some(desc) = op.description {
                o.insert("docstring".to_string(), to_json(&desc));
            }
            if let Some(tt) = op.tags {
                for t in tt {
                    let name = fix_name(t);
                    ops_tags.insert(name.clone());
                    if !ops.contains_key(&name) {
                        let mut m = serde_json::Map::new();
                        m.insert("get".to_string(), to_json(&serde_json::Map::new()));
                        ops.insert(name.clone(), to_json(&m));
                    }
                    let g = ops.get_mut(&name).unwrap().as_object_mut().unwrap().get_mut("get").unwrap().as_object_mut().unwrap();
                    g.insert(fname.clone(), to_json(&o));
                }
            } else {
                let g = ops.get_mut("generic").unwrap().as_object_mut().unwrap();
                g.insert(fname, to_json(&o));
            }
        }
    }
    root_map.insert("ops_tags".to_string(), to_json(&ops_tags));
    root_map.insert("ops".to_string(), to_json(&ops));

    let mut types_tags = BTreeSet::<String>::new();
    types_tags.insert("generic".to_string());
    // Parse types
    // TODO
    root_map.insert("types_tags".to_string(), to_json(&types_tags));

    let ctx = handlebars::Context::wraps(&root_map);

    // Write types-related files
    {
        let typesmodf = std::fs::File::create(&cfg.out_dir.join("types").join("mod.rs"))?;
        hs.renderw(
            "types_mod",
            &root_map,
            &mut std::io::BufWriter::new(typesmodf),
        ).chain_err(|| "failed rendering types_mod")?;
        for t in &types_tags {
            let typesf = std::fs::File::create(cfg.out_dir.join("types").join(t.clone() + ".rs"))?;
        }
    }


    // Write ops-related files
    {
        let modf = std::fs::File::create(&cfg.out_dir.join("mod.rs"))?;
        hs.renderw("mod", &root_map, &mut std::io::BufWriter::new(modf))
            .chain_err(|| "failed rendering mod")?;
        for t in &ops_tags {
            let getf = std::fs::File::create(cfg.out_dir.join(t.clone() + ".rs"))?;
            let get_ops = ctx.navigate(
                ".",
                &VecDeque::new(),
                &format!("ops.{}", t),
            )?;
            hs.renderw("get", &get_ops, &mut std::io::BufWriter::new(getf))
                .chain_err(|| "failed rendering get")?;
        }
    }
    Ok(())
}
