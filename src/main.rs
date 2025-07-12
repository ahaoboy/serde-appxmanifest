use serde_appxmanifest::Package;
use serde_xml_rs::from_str;

fn main() {
    let Some(p) = std::env::args().nth(1) else {
        println!("serde-appxmanifest <AppxManifest.xml>");
        return;
    };

    let xml = std::fs::read_to_string(&p).expect("failed to read xml file");

    let package: Package = from_str(&xml).expect("failed to parse xml");

    println!("{package:?}");
}
