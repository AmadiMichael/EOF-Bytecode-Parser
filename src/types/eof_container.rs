use crate::types::full_code_section::FullCodeSection;
use std::fmt::Display;

#[derive(Debug)]
pub struct EOFContainer {
    pub header: Header,
    pub full_code_section: Vec<FullCodeSection>,
    pub container_section: Vec<Vec<u8>>,
    pub data_section: Vec<u8>,
}

#[derive(Debug)]
pub struct Header {
    pub version: u8,
    pub types_size: u16,
    pub num_code_sections: u16,
    pub code_sizes: Vec<u16>,
    pub num_container_sections: u16,
    pub container_sizes: Vec<u16>,
    pub data_size: u16,
}

impl Display for EOFContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data_section = String::from("0x");
        data_section.push_str(&hex::encode(&self.data_section));

        let mut container_sections = String::new();
        if self.container_section.len() == 0 {
            container_sections.push_str(
                "
            CONTAINER_SECTION: None
            ",
            );
        } else {
            for (i, container_section) in self.container_section.iter().enumerate() {
                container_sections.push_str(&format!(
                    "
            CONTAINER_SECTION_{}: 0x{}
                    ",
                    i,
                    hex::encode(container_section),
                ));
            }
        }

        write!(
            f,
            "
            HEADER:
                version: {},
                types_size: {}
                num_code_sections: {},
                code_sizes: {:?},
                num_container_sections: {},
                container_sizes: {:?},
                data_size: {}
            {}
            {}
            DATA_SECTION: {}
            ",
            self.header.version,
            self.header.types_size,
            self.header.num_code_sections,
            self.header.code_sizes,
            self.header.num_container_sections,
            self.header.container_sizes,
            self.header.data_size,
            FullCodeSection::fmt(&self.full_code_section),
            container_sections,
            data_section
        )
    }
}

impl Default for EOFContainer {
    fn default() -> Self {
        EOFContainer {
            header: Header {
                version: 0,
                types_size: 0,
                num_code_sections: 0,
                code_sizes: Vec::new(),
                num_container_sections: 0,
                container_sizes: Vec::new(),
                data_size: 0,
            },
            full_code_section: Vec::new(),
            container_section: Vec::new(),
            data_section: Vec::new(),
        }
    }
}
