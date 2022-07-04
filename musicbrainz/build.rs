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
    #[serde(skip)]
    #[allow(dead_code)]
    pva: (),
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

#[derive(Deserialize)]
struct ISO3166 {
    country: String,
    alpha2: String,
    alpha3: Option<String>,
    numeric: Option<String>,
}

fn generate_scripts<P: AsRef<Path>>(out: P) -> Result<()> {
    let asset = Path::new("./assets/iso-15924.json");
    println!("cargo:rerun-if-changed={}", asset.to_string_lossy());
    let asset = File::open(asset).context("opening script list")?;
    let reader = BufReader::new(asset);
    serde_json::from_reader::<_, Vec<ISO15924>>(reader)
        .context("parse script list")?
        .iter()
        .map(|l| {
            let variant: syn::Variant =
                syn::parse_str(&l.code.to_camel()).context("parse script code")?;
            let name = format!("# {}", l.name);
            let numeric = format!("* Numeric: {}", l.numeric);
            let date = format!("* Date: {}", l.date);
            Ok(quote! {
                #[doc = #name]
                #[doc = #numeric]
                #[doc = #date]
                #variant,
            })
        })
        .try_fold::<_, _, Result<_>>(TokenStream::new(), |mut ts, variant: Result<_>| {
            ts.extend(variant?);
            anyhow::Result::Ok(ts)
        })
        .map(|ts| {
            quote! {
                /// ISO 15924 Scripts
                #[derive(Debug, serde::Deserialize, serde::Serialize)]
                pub enum Script {
                    #ts
                }
            }
        })
        .and_then(|tks| syn::parse2(tks).context("parse generated scripts source"))
        .map(|f| prettyplease::unparse(&f))
        .and_then(|src| std::fs::write(&out, &src).context("write scripts source"))?;

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
                #[derive(Debug, serde::Deserialize, serde::Serialize)]
                #[serde(rename_all = "lowercase")]
                pub enum Language {
                    #ts
                }
            }
        })
        .and_then(|tks| syn::parse2(tks).context("parse generated language source"))
        .map(|f| prettyplease::unparse(&f))
        .and_then(|src| std::fs::write(&out, &src).context("write languages source"))?;

    Ok(())
}

fn generate_countries<P: AsRef<Path>>(out: P) -> Result<()> {
    let asset = Path::new("./assets/iso-3166.json");
    println!("cargo:rerun-if-changed={}", asset.to_string_lossy());
    let asset = File::open(asset).context("opening country list")?;
    let reader = BufReader::new(asset);
    serde_json::from_reader::<_, Vec<ISO3166>>(reader)
        .context("parse country list")?
        .iter()
        .map(|l| {
            let variant: syn::Variant =
                syn::parse_str(&l.alpha2.to_camel()).context("parse country alpha2")?;
            let country = format!("# {}", l.country);
            let alpha2 = format!("* Alpha-2: {}", l.alpha2);
            let alpha3 = format!("* Alpha-3: {}", l.alpha3.as_ref().unwrap_or(&"None".to_string()));
            let numeric = format!("* Numeric: {}", l.numeric.as_ref().unwrap_or(&"None".to_string()));
            Ok(quote! {
                #[doc = #country]
                #[doc = #alpha2]
                #[doc = #alpha3]
                #[doc = #numeric]
                #variant,
            })
        })
        .try_fold::<_, _, Result<_>>(TokenStream::new(), |mut ts, variant: Result<_>| {
            ts.extend(variant?);
            anyhow::Result::Ok(ts)
        })
        .map(|ts| {
            quote! {
                /// ISO 3166 Alpha-2 Country Codes
                #[derive(Debug, serde::Deserialize, serde::Serialize)]
                #[serde(rename_all = "UPPERCASE")]
                pub enum Country {
                    #ts
                }
            }
        })
        .and_then(|tks| syn::parse2(tks).context("parse generated country source"))
        .map(|f| prettyplease::unparse(&f))
        .and_then(|src| std::fs::write(&out, &src).context("write country source"))?;

    Ok(())
}

fn main() -> Result<()> {
    let out_root = var("OUT_DIR").context("get OUT_DIR")?;
    generate_languages(format!("{out_root}/languages.rs"))?;
    generate_scripts(format!("{out_root}/scripts.rs"))?;
    generate_countries(format!("{out_root}/countries.rs"))?;
    Ok(())
}
