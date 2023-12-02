use std::{
    env::args,
    io::{self, Write},
};

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

    define_ast(
        output_dir,
        "Expr",
        &[
            "Assign   : Token name, Box<Expr> value",
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping : Box<Expr> expression",
            "Literal  : Option<Literal> value",
            "Unary    : Token operator, Box<Expr> right",
            "Variable : Token name",
        ],
        &["error", "token", "literal"],
    )?;
    define_ast(
        output_dir,
        "Stmt",
        &[
            "Block  : Vec<Stmt> statements",
            "Expression : Expr expression",
            "If       : Expr condition, Box<Stmt> then_branch, Option<Box<Stmt>>  else_branch",
            "Print : Expr expression",
            "Var : Token name, Option<Expr> initializer",
        ],
        &["error", "expr", "token"],
    )?;
    Ok(())
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    types: &[&str],
    includes: &[&str],
) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let file = std::fs::File::create(path).unwrap();
    let tree_types = prepare_treetypes(types, base_name);

    prepare_imports(&file, includes)?;

    prepare_enum(&file, &tree_types, base_name)?;

    prepare_structs(&file, &tree_types)?;

    prepare_trait(&file, &tree_types, base_name)?;

    prepare_visitors(&file, &tree_types, base_name)?;

    Ok(())
}

fn prepare_imports(mut file: &std::fs::File, includes: &[&str]) -> Result<(), io::Error> {
    for incl in includes {
        writeln!(file, "use crate::{}::*;", incl)?;
    }
    Ok(())
}

fn prepare_treetypes(types: &[&str], base_name: &str) -> Vec<TreeType> {
    let mut tree_types = Vec::<TreeType>::new();
    for ttype in types {
        let (base_class, fields) = ttype.split_once(':').unwrap();
        let class_name = format!("{}{}", base_name.trim(), base_class);
        let args = fields.split(',');
        let mut field_names = Vec::new();

        for arg in args {
            let (field_type, name) = arg.trim().split_once(' ').unwrap();
            field_names.push(format!("{}: {},\n", name, field_type));
        }
        tree_types.push(TreeType {
            base_class: base_class.trim().to_string(),
            class_name: class_name.trim().to_string(),
            fields: field_names,
        })
    }
    tree_types
}

fn prepare_enum(
    mut file: &std::fs::File,
    tree_types: &Vec<TreeType>,
    base_name: &str,
) -> Result<(), io::Error> {
    writeln!(file, "\n pub enum {base_name} {{")?;
    for t in tree_types {
        writeln!(file, "    {}({}),", t.base_class, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {base_name} {{")?;

    writeln!(file,"    pub fn accept<T>(&self, visitor: &dyn {base_name}Visitor<T>) -> Result<T, LoxError> {{")?;

    writeln!(file, "        match self {{")?;

    for t in tree_types {
        writeln!(file,
            "        {base_name}::{base_class}({class_name}) => visitor.visit_{base_class_lc}_{base_name_lc}({class_name}),",
            base_name = base_name,
            base_class = t.base_class,
            class_name = format!("{}_{}",t.base_class.to_lowercase(), base_name.to_lowercase() ),
            base_class_lc = t.base_class.to_lowercase(),
            base_name_lc = base_name.to_lowercase(),
            )?;
    }

    writeln!(file, "        }}")?;

    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    Ok(())
}

fn prepare_structs(mut file: &std::fs::File, tree_types: &Vec<TreeType>) -> Result<(), io::Error> {
    for t in tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for field in &t.fields {
            write!(file, "   pub {}", field)?;
        }
        writeln!(file, "}}\n")?;
    }
    Ok(())
}

fn prepare_trait(
    mut file: &std::fs::File,
    tree_types: &Vec<TreeType>,
    base_name: &str,
) -> Result<(), io::Error> {
    writeln!(file, "pub trait {base_name}Visitor<T> {{")?;
    for t in tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError> ;",
            t.base_class.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    writeln!(file, "}}\n")?;
    Ok(())
}

fn prepare_visitors(
    mut file: &std::fs::File,
    tree_types: &Vec<TreeType>,
    base_name: &str,
) -> Result<(), io::Error> {
    for t in tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_class.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
    }
    Ok(())
}
