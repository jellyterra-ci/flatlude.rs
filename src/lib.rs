// Authored 2026 Jelly Terra <jellyterra@proton.me>

use proc_macro::TokenStream;
use quote::quote;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn fetch_mods(dir: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let src_dir = dir.join("src");

    let mut mods = Vec::new();

    let entries = match fs::read_dir(&src_dir) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to read src directory [{}]: {}", src_dir.display(), e).into()),
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "rs" {
                    if let Some(stem) = path.file_stem() {
                        let mod_name = stem.to_string_lossy().to_string();
                        if mod_name != "lib" && mod_name != "main" {
                            mods.push(mod_name);
                        }
                    }
                }
            }
        } else if PathBuf::from(&path).join("mod.rs").exists() {
            let mod_name = path.file_stem().unwrap().to_string_lossy().to_string();
            mods.push(mod_name);
        }
    }

    Ok(mods)
}

#[proc_macro]
pub fn flatlude(_input: TokenStream) -> TokenStream {
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(path) => path,
        Err(_) => {
            return TokenStream::from(quote! { compile_error!("Unable to read CARGO_MANIFEST_DIR environment variable."); });
        }
    };

    let mut module_declarations = vec![];

    for mod_name in fetch_mods(&PathBuf::from(&manifest_dir)).unwrap() {
        let mod_name = syn::Ident::new(&mod_name, proc_macro2::Span::call_site());
        module_declarations.push(quote! {
            pub mod #mod_name;
            pub use #mod_name::*;
        });
    }

    let expanded = quote! {
        #(#module_declarations)*
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn mods(_input: TokenStream) -> TokenStream {
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(path) => path,
        Err(_) => {
            return TokenStream::from(quote! { compile_error!("Unable to read CARGO_MANIFEST_DIR environment variable."); });
        }
    };

    let mut module_declarations = vec![];

    for mod_name in fetch_mods(&PathBuf::from(&manifest_dir)).unwrap() {
        let mod_name = syn::Ident::new(&mod_name, proc_macro2::Span::call_site());
        module_declarations.push(quote! {
            pub mod #mod_name;
        });
    }

    let expanded = quote! {
        #(#module_declarations)*
    };

    TokenStream::from(expanded)
}
