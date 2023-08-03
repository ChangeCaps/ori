use std::{env, fs, io, path::Path};

use quote::quote;
use syn::LitStr;

use crate::krate::find_crate;

fn load_dir(path: &Path) -> io::Result<String> {
    let mut style = String::new();

    for entry in path.read_dir()? {
        let entry = entry?;
        let input = load_style(&entry.path())?;
        style.push_str(&input);
    }

    Ok(style)
}

fn load_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn load_style(path: &Path) -> io::Result<String> {
    if path.is_dir() {
        load_dir(path)
    } else {
        load_file(path)
    }
}

pub fn style(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    let input: LitStr = syn::parse(input)?;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);
    let style_path = manifest_dir.join(input.value());

    let style = match load_style(&style_path) {
        Ok(style) => style,
        Err(err) => manyhow::bail!(input, "io error: {}", err),
    };

    let ori_style = find_crate("style");
    let ori_core = find_crate("core");

    let style_path = style_path.to_str().unwrap();
    let expanded = quote! {
        if let Ok(sheet) = #ori_style::LoadedStyle::load(#style_path) {
            #ori_style::LoadedStyleKind::Loaded(sheet)
        } else {
            let sheet = ::std::str::FromStr::from_str(#style).unwrap_or_else(|err| {
                #ori_core::tracing::error!("failed to parse style: {}", err);
                #ori_style::Stylesheet::empty()
            });

            #ori_style::LoadedStyleKind::Inline(sheet)
        }
    };

    Ok(expanded.into())
}
