use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use log::debug;

// optionally what might be a good exercise is to create a macro that
// I can throw on structs that will just implement the Expr trait on them?

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn define_visitor(f: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {
    f.write("pub trait Visitor<T>{\n".as_bytes())?;

    //
    for type_string in types {
        let type_name = type_string
            .split(".")
            .nth(0)
            .expect("to get struct name")
            .trim();

        let write_string = if type_name == "Literal" {
            format!(
                "fn visit_{}_{}(&self, {}: &{}) -> T;\n",
                type_name,
                base_name,
                base_name.to_lowercase(),
                type_name
            )
        } else {
            format!(
                "fn visit_{}_{}(&self, {}: &{}<T>) -> T;\n",
                type_name,
                base_name,
                base_name.to_lowercase(),
                type_name
            )
        };

        f.write(write_string.as_bytes())?;
    }

    f.write(b"}")?;

    Ok(())
}

fn define_type(f: &mut File, struct_name: &str, fields: &str, base_name: &str) -> Result<()> {
    if struct_name == "Literal" {
        f.write(format!("pub struct {} {{\n", struct_name).as_bytes())?;
    } else {
        f.write(format!("pub struct {}<T> {{\n", struct_name).as_bytes())?;
    }

    for field in fields.split(",") {
        let field = field.trim();
        f.write(format!("pub {},\n", field).as_bytes())?;
    }

    f.write("}\n".as_bytes())?;

    let write_string = if struct_name == "Literal" {
        format!(
            r#"impl<T> Expr<T> for {} {{
                fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {{
                    return visitor.visit_{}_{}(self);
                }}
}}"#,
            struct_name, struct_name, base_name
        )
    } else {
        format!(
            r#"impl<T> Expr<T> for {}<T> {{
                fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {{
                    return visitor.visit_{}_{}(self);
                }}
}}"#,
            struct_name, struct_name, base_name
        )
    };
    f.write(write_string.as_bytes())?;
    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    debug!("creating file {}", &path);
    let p: PathBuf = PathBuf::from(path);

    let mut f = OpenOptions::new().write(true).create(true).open(p)?;

    f.write(format!("use crate::parser::{{Token, Object}};\n").as_bytes())?;
    f.write(
        format!(
            r#"pub trait {}<T> {{
        fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T;
}}
"#,
            base_name
        )
        .as_bytes(),
    )?;
    // file automatically closed when it goes out of scope

    for type_of in &types {
        let struct_name = type_of
            .split(".")
            .nth(0)
            .expect("a struct name from 'types'")
            .trim();
        let fields = type_of
            .split(".")
            .nth(1)
            .expect("a struct name from 'types'")
            .trim();
        define_type(&mut f, struct_name, fields, base_name)?;
    }

    define_visitor(&mut f, base_name, &types)?;

    Ok(())
}

pub fn generate_exprs() -> Result<()> {
    define_ast(
        "src",
        "Expr",
        vec![
            "Binary . left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>",
            "Grouping . expression: Box<dyn Expr<T>>",
            "Literal . value: Object",
            "Unary . operator: Token, right: Box<dyn Expr<T>>",
        ],
    )
}
