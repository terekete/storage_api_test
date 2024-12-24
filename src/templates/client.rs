use crate::templates::TemplateData;
use anyhow::Result;
use std::collections::HashMap;
use tera::{Context, Tera};

pub struct TemplateRenderer {
    tera: Tera,
    data: TemplateData,
}

impl TemplateRenderer {
    pub fn new(template_dir: &str, data: TemplateData) -> Result<Self> {
        let mut tera = Tera::default();
        tera.add_template_files(vec![(
            format!("{}/**/*.tera", template_dir),
            Some("templates"),
        )])?;

        Ok(Self { tera, data })
    }

    pub fn render(&self, template_name: &str) -> Result<String> {
        let mut context = Context::new();
        context.insert("api", &self.data);

        Ok(self.tera.render(template_name, &context)?)
    }

    pub fn render_with_context(
        &self,
        template_name: &str,
        extra_context: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let mut context = Context::new();
        context.insert("api", &self.data);

        for (key, value) in extra_context {
            context.insert(key.as_str(), &value);
        }

        Ok(self.tera.render(template_name, &context)?)
    }
}
