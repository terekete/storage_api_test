use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tera::{Context as TeraContext, Tera};

use crate::discovery::ApiDefinition;

pub struct Generator {
    tera: Tera,
    output_dir: PathBuf,
    api_def: ApiDefinition,
}

impl Generator {
    pub fn new<P: AsRef<Path>>(
        template_dir: P,
        output_dir: P,
        api_def: ApiDefinition,
    ) -> Result<Self> {
        // Create a new Tera instance
        let mut tera = Tera::default();

        // Convert template directory to absolute path
        let template_dir = fs::canonicalize(template_dir.as_ref())
            .context("Failed to get absolute path for template directory")?;

        let pattern = template_dir
            .join("**")
            .join("*.tera")
            .to_string_lossy()
            .into_owned();

        println!("Looking for templates in: {}", pattern);

        // First ensure the template directory exists
        if !template_dir.exists() {
            return Err(anyhow::anyhow!(
                "Template directory does not exist: {}",
                template_dir.display()
            ));
        }

        // List all .tera files in the template directory
        let template_files: Vec<_> = walkdir::WalkDir::new(&template_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "tera"))
            .collect();

        println!("Found template files:");
        for file in &template_files {
            println!("  - {}", file.path().display());
        }

        // Add each template file individually
        for entry in template_files {
            let path = entry.path();
            let relative_path = path.strip_prefix(&template_dir)?;
            let template_name = relative_path.to_string_lossy().into_owned();

            println!("Adding template: {} from {}", template_name, path.display());

            let template_content = fs::read_to_string(path)
                .context(format!("Failed to read template file: {}", path.display()))?;

            tera.add_raw_template(&template_name, &template_content)
                .context(format!("Failed to add template: {}", template_name))?;
        }

        // Add custom filters
        tera.register_filter("to_rust_type", Self::to_rust_type);
        tera.register_filter("to_snake_case", Self::to_snake_case);
        tera.register_filter("to_pascal_case", Self::to_pascal_case);

        Ok(Self {
            tera,
            output_dir: output_dir.as_ref().to_path_buf(),
            api_def,
        })
    }

    pub fn generate(&self) -> Result<()> {
        let mut context = TeraContext::new();
        context.insert("api", &self.api_def);

        // Generate library crate
        self.generate_lib_crate(&context)?;

        // Generate CLI crate
        self.generate_cli_crate(&context)?;

        Ok(())
    }

    fn generate_lib_crate(&self, context: &TeraContext) -> Result<()> {
        let lib_dir = self.get_lib_dir();
        fs::create_dir_all(&lib_dir)?;

        // Generate Cargo.toml
        self.render_template("api/Cargo.toml.tera", &lib_dir.join("Cargo.toml"), context)?;

        // Generate source files
        let src_dir = lib_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        for template in &[
            "lib.rs.tera",
            "client.rs.tera",
            "types.rs.tera",
            "error.rs.tera",
        ] {
            let output = src_dir.join(template.replace(".tera", ""));
            self.render_template(&format!("api/src/{}", template), &output, context)?;
        }

        Ok(())
    }

    fn generate_cli_crate(&self, context: &TeraContext) -> Result<()> {
        let cli_dir = self.get_cli_dir();
        fs::create_dir_all(&cli_dir)?;

        // Generate Cargo.toml
        self.render_template("cli/Cargo.toml.tera", &cli_dir.join("Cargo.toml"), context)?;

        // Generate source files
        let src_dir = cli_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        self.render_template("cli/src/main.rs.tera", &src_dir.join("main.rs"), context)?;

        Ok(())
    }

    fn render_template<P: AsRef<Path>>(
        &self,
        template: &str,
        output: P,
        context: &TeraContext,
    ) -> Result<()> {
        let content = self
            .tera
            .render(template, context)
            .context(format!("Failed to render template {}", template))?;

        fs::write(output, content).context(format!("Failed to write file {}", template))?;

        Ok(())
    }

    fn get_lib_dir(&self) -> PathBuf {
        self.output_dir
            .join(format!("storage{}", self.api_def.version))
    }

    fn get_cli_dir(&self) -> PathBuf {
        self.output_dir
            .join(format!("storage{}-cli", self.api_def.version))
    }

    fn to_rust_type(
        value: &Value,
        _: &std::collections::HashMap<String, Value>,
    ) -> tera::Result<Value> {
        let type_str = value.as_str().unwrap_or("string");
        let rust_type = match type_str {
            "string" => "String",
            "integer" => "i64",
            "int32" => "i32",
            "int64" => "i64",
            "boolean" => "bool",
            "number" => "f64",
            "float" => "f32",
            "double" => "f64",
            "array" => "Vec<String>", // Default to Vec<String>, should be overridden when array type is known
            "object" => "serde_json::Value",
            _ => "String", // Default to String for unknown types
        };
        Ok(Value::String(rust_type.to_string()))
    }

    fn to_snake_case(
        value: &Value,
        _: &std::collections::HashMap<String, Value>,
    ) -> tera::Result<Value> {
        Ok(Value::String(
            value.as_str().unwrap_or("").to_case(Case::Snake),
        ))
    }

    fn to_pascal_case(
        value: &Value,
        _: &std::collections::HashMap<String, Value>,
    ) -> tera::Result<Value> {
        Ok(Value::String(
            value.as_str().unwrap_or("").to_case(Case::Pascal),
        ))
    }
}
