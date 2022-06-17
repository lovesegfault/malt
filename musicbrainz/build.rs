use std::io::BufReader;
use std::path::Path;
use std::{env::var, fs::File};

use anyhow::{Context, Result};
use case::CaseExt;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct ISO15924 {
    code: String,
    name: String,
    numeric: String,
    pva: String,
    date: String,
}

#[derive(Deserialize)]
struct ISO6393 {
    name: String,
    #[serde(rename = "type")]
    ltype: String,
    scope: String,
    iso6393: String,
}

fn sanitize_variant(name: &str) -> String {
    name.chars()
        .filter_map(|c| match c {
            '\'' => Some('_'),
            c if c.is_alphanumeric() => Some(c),
            _ => None,
        })
        .collect::<String>()
        .to_camel()
}

fn generate_scripts() -> Result<()> {
    Ok(())
}

fn generate_languages<P: AsRef<Path>>(out: P) -> Result<()> {
    let asset = Path::new("./assets/iso-639-3.json");
    println!("cargo:rerun-if-changed={}", asset.to_string_lossy());
    let asset = File::open(asset).context("opening language list")?;
    let reader = BufReader::new(asset);
    serde_json::from_reader::<_, Vec<ISO6393>>(reader)
        .context("parse language list")?
        .iter()
        .map(|l| {
            let name: syn::Variant = syn::parse_str(&sanitize_variant(&l.name))
                .context("parse ISO 693-3 language name")?;
            let msg = format!(
                r#"
/// {}
/// * Type: {}
/// * Scope: {}
/// * ISO 693-3: {}"#,
                l.name, l.ltype, l.scope, l.iso6393
            );
            Ok(quote! {
                #[doc = #msg]
                #name,
            })
        })
        .try_fold::<_, _, Result<_>>(TokenStream::new(), |mut ts, variant: Result<_>| {
            ts.extend(variant?);
            anyhow::Result::Ok(ts)
        })
        .map(|ts| {
            quote! {
                /// ISO 693-3 Languages
                /// FIXME
                pub enum Languages {
                    #ts
                }
            }
        })
        .and_then(|tks| syn::parse2(tks).context("parse generated language source"))
        .map(|f| prettyplease::unparse(&f))
        .and_then(|src| std::fs::write(&out, &src).context("write languages source"))?;

    Ok(())
}

fn main() -> Result<()> {
    let out_root = var("OUT_DIR").context("get OUT_DIR")?;
    generate_languages(out_root + "/languages.rs")?;
    Ok(())
}
