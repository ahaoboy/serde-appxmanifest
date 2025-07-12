use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Package")]
pub struct Package {
    #[serde(rename = "Identity")]
    identity: Identity,
    #[serde(rename = "Properties")]
    properties: Properties,
    #[serde(rename = "Dependencies")]
    dependencies: Dependencies,
    // FIXME: duplicate field `rescap:Capability`
    // #[serde(rename = "Capabilities")]
    // capabilities: Vec<Capabilities>,
    #[serde(rename = "Applications")]
    applications: Applications,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identity {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@ProcessorArchitecture")]
    processor_architecture: Option<String>,
    #[serde(rename = "@Publisher")]
    publisher: String,
    #[serde(rename = "@Version")]
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    #[serde(rename = "DisplayName")]
    display_name: String,
    #[serde(rename = "PublisherDisplayName")]
    publisher_display_name: String,
    #[serde(rename = "Logo")]
    logo: String,
    #[serde(rename = "uap10:AllowExternalContent")]
    allow_external_content: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    #[serde(rename = "TargetDeviceFamily")]
    target_device_family: Vec<TargetDeviceFamily>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetDeviceFamily {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@MinVersion")]
    min_version: String,
    #[serde(rename = "@MaxVersionTested")]
    max_version_tested: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "rescap:Capability")]
    capability: Vec<Capability>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Capability {
    #[serde(rename = "@Name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Applications {
    #[serde(rename = "Application")]
    application: Vec<Application>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "@Id")]
    id: String,
    #[serde(rename = "uap10:TrustLevel")]
    trust_level: Option<String>,
    #[serde(rename = "uap10:RuntimeBehavior")]
    runtime_behavior: Option<String>,
    #[serde(rename = "uap:VisualElements")]
    visual_elements: VisualElements,
    #[serde(rename = "Extensions")]
    extensions: Option<Extensions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VisualElements {
    #[serde(rename = "@AppListEntry")]
    app_list_entry: Option<String>,
    #[serde(rename = "@DisplayName")]
    display_name: String,
    #[serde(rename = "@Description")]
    description: String,
    #[serde(rename = "@BackgroundColor")]
    background_color: String,
    #[serde(rename = "@Square150x150Logo")]
    square_150x150_logo: String,
    #[serde(rename = "@Square44x44Logo")]
    square_44x44_logo: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Extensions {
    #[serde(rename = "desktop4:Extension")]
    desktop_extension: Option<Vec<DesktopExtension>>,
    #[serde(rename = "com:Extension")]
    com_extension: Option<Vec<ComExtension>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DesktopExtension {
    #[serde(rename = "desktop4:FileExplorerContextMenus")]
    file_explorer_context_menus: FileExplorerContextMenus,
}

#[derive(Debug, Deserialize, Serialize)]
struct FileExplorerContextMenus {
    #[serde(rename = "desktop5:ItemType")]
    item_type: Vec<ItemType>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ItemType {
    #[serde(rename = "@Type")]
    ty: String,
    #[serde(rename = "desktop5:Verb")]
    verb: Verb,
}

#[derive(Debug, Deserialize, Serialize)]
struct Verb {
    #[serde(rename = "@Id")]
    id: String,
    #[serde(rename = "@Clsid")]
    clsid: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComExtension {
    #[serde(rename = "com:ComServer")]
    com_server: Option<ComServer>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComServer {
    #[serde(rename = "com:SurrogateServer")]
    surrogate_server: Option<SurrogateServer>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SurrogateServer {
    #[serde(rename = "@DisplayName")]
    display_name: String,
    #[serde(rename = "com:Class")]
    com_class: Vec<ComClass>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComClass {
    #[serde(rename = "@Id")]
    id: String,
    #[serde(rename = "@Path")]
    path: String,
    #[serde(rename = "@ThreadingModel")]
    threading_model: String,
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
