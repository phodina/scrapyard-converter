error_chain!{
        foreign_links {
            Io(::std::io::Error);
            SerdeXML(::serde_xml_rs::Error);
            SerdeJSON(::serde_json::Error);
            Parse(::std::num::ParseIntError);
        }
    }