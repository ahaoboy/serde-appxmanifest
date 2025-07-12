use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Package")]
pub struct Package {
    #[serde(rename = "Identity")]
    pub identity: Identity,
    #[serde(rename = "Properties")]
    pub properties: Properties,
    #[serde(rename = "Dependencies")]
    pub dependencies: Dependencies,
    // FIXME: duplicate field `rescap:Capability`
    // #[serde(rename = "Capabilities")]
    // capabilities: Vec<Capabilities>,
    #[serde(rename = "Applications")]
    pub applications: Applications,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identity {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@ProcessorArchitecture")]
    pub processor_architecture: Option<String>,
    #[serde(rename = "@Publisher")]
    pub publisher: String,
    #[serde(rename = "@Version")]
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "PublisherDisplayName")]
    pub publisher_display_name: String,
    #[serde(rename = "Logo")]
    pub logo: String,
    #[serde(rename = "uap10:AllowExternalContent")]
    pub allow_external_content: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    #[serde(rename = "TargetDeviceFamily")]
    pub target_device_family: Vec<TargetDeviceFamily>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetDeviceFamily {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@MinVersion")]
    pub min_version: String,
    #[serde(rename = "@MaxVersionTested")]
    pub max_version_tested: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "rescap:Capability")]
    pub capability: Vec<Capability>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Capability {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Applications {
    #[serde(rename = "Application")]
    pub application: Vec<Application>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "uap10:TrustLevel")]
    pub trust_level: Option<String>,
    #[serde(rename = "uap10:RuntimeBehavior")]
    pub runtime_behavior: Option<String>,
    #[serde(rename = "uap:VisualElements")]
    pub visual_elements: VisualElements,
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VisualElements {
    #[serde(rename = "@AppListEntry")]
    pub app_list_entry: Option<String>,
    #[serde(rename = "@DisplayName")]
    pub display_name: String,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@BackgroundColor")]
    pub background_color: String,
    #[serde(rename = "@Square150x150Logo")]
    pub square_150x150_logo: String,
    #[serde(rename = "@Square44x44Logo")]
    pub square_44x44_logo: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Extensions {
    #[serde(rename = "desktop4:Extension")]
    pub desktop_extension: Option<Vec<DesktopExtension>>,
    #[serde(rename = "com:Extension")]
    pub com_extension: Option<Vec<ComExtension>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DesktopExtension {
    #[serde(rename = "desktop4:FileExplorerContextMenus")]
    pub file_explorer_context_menus: FileExplorerContextMenus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileExplorerContextMenus {
    #[serde(rename = "desktop5:ItemType")]
    pub item_type: Vec<ItemType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemType {
    #[serde(rename = "@Type")]
    pub ty: String,
    #[serde(rename = "desktop5:Verb")]
    pub verb: Verb,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Verb {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Clsid")]
    pub clsid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComExtension {
    #[serde(rename = "com:ComServer")]
    pub com_server: Option<ComServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComServer {
    #[serde(rename = "com:SurrogateServer")]
    pub surrogate_server: Option<SurrogateServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurrogateServer {
    #[serde(rename = "@DisplayName")]
    pub display_name: String,
    #[serde(rename = "com:Class")]
    pub com_class: Vec<ComClass>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComClass {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Path")]
    pub path: String,
    #[serde(rename = "@ThreadingModel")]
    pub threading_model: String,
}
#[cfg(test)]
mod test {
    use std::io::read_to_string;

    use serde_xml_rs::from_str;

    use crate::Package;

    #[test]
    fn test_xml() {
        for (i) in std::fs::read_dir("assets/xml").unwrap() {
            let xml = std::fs::read_to_string(i.unwrap().path()).unwrap();
            let package = from_str::<Package>(&xml).unwrap();
            assert!(!package.identity.name.is_empty());
        }
    }
}
