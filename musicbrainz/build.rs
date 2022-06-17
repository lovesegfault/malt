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
            let variant: syn::Variant =
                syn::parse_str(&l.iso6393.to_camel()).context("parse language code")?;
            let entonym = format!("# {}", l.name);
            let ltype = format!("* Type: {}", l.ltype);
            let scope = format!("* Scope: {}", l.scope);
            let code = format!("* ISO 639-3: {}", l.iso6393);
            Ok(quote! {
                #[doc = #entonym]
                #[doc = #ltype]
                #[doc = #scope]
                #[doc = #code]
                #variant,
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
