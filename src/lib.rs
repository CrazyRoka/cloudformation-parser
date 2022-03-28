use mapping::Mapping;
use output::Output;
use parameter::Parameter;
use resources::Resource;
use serde::Deserialize;
use std::collections::HashMap;

mod data_type;
mod mapping;
mod output;
mod parameter;
mod resources;
mod tag;
mod value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    #[serde(rename = "AWSTemplateFormatVersion")]
    aws_template_format_version: Option<String>,
    metadata: Option<HashMap<String, String>>,
    description: Option<String>,
    mappings: Option<Mapping>,
    parameters: Option<HashMap<String, Parameter>>,
    resources: HashMap<String, Resource>,
    outputs: Option<HashMap<String, Output>>,
}
