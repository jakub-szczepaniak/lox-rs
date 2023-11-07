use std::{env::args, io::{self, Write}};

struct TreeType {
    base_class: String,
    class_name: String,
    fields: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: lox-gen-ast <output directory>");
        std::process::exit(64);
    }
    let output_dir = &args[1];
    define_ast(output_dir, "Expr", &[
        "Binary   : Box<Expr> left, Token operator, Box<Expr> right",
        "Grouping : Box<Expr> expression",
        "Literal  : Literal value",
        "Unary    : Token operator, Box<Expr> right",
    ])?;
    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = std::fs::File::create(path).unwrap();
    let mut tree_types = Vec::<TreeType>::new();
    writeln!(file, "use crate::error::*;")?;

    write!(file, "use crate::token::*;\n")?;

    for ttype in types {
        let (base_class, fields) = ttype.split_once(":").unwrap();
        println!("base_class: {}, fields: {}", base_class, fields);
        let class_name = format!("{}{}", base_name.trim(), base_class);
        println!("class_name: {}", class_name);
        let args  = fields.split(",");
        let mut field_names = Vec::new();
        for arg in args {
            let (field_type, name) = arg.trim().split_once(" ").unwrap();
            field_names.push(format!("{}: {},\n", name, field_type));
        }
        tree_types.push(TreeType { 
            base_class: base_class.trim().to_string(), 
            class_name: class_name.trim().to_string(), 
            fields: field_names})
    }
    write!(file,"\n pub enum {base_name} {{ \n")?;
    for t in &tree_types {
        write!(file,"    {}({}),\n", t.base_class, t.class_name)?;
    }
    write!(file,"}}\n\n")?;
    
    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for field in &t.fields {
            write!(file, "    {}", field)?;
        }
        write!(file,"}}\n\n")?;
    }

    writeln!(file, "pub trait ExprVisitor<T> {{")?;

    for t in &tree_types {
        writeln!(file, "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError> {{}}",
    t.base_class.to_lowercase(),
    base_name.to_lowercase(),
    t.class_name)?;

    }

    writeln!(file,"}}\n")?;

    for t in &tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(file, "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{", base_name);
        writeln!(file, "        visitor.visit_{}_{}(self)", t.base_class.to_lowercase(), base_name.to_lowercase())?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }
    Ok(())
}