use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FullCodeSection {
    pub types_section: TypesSection,
    pub code_section: Vec<u8>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TypesSection {
    pub inputs: u8,
    pub outputs: Option<u8>,
    pub max_stack_height: u16,
}

impl FullCodeSection {
    pub fn fmt(arr: &[FullCodeSection]) -> String {
        let mut formatted = String::new();

        for (i, full_code_section) in arr.iter().enumerate() {
            let mut code_section = String::from("0x");
            code_section.push_str(&hex::encode(&full_code_section.code_section));

            let outputs;
            if let Some(x) = full_code_section.types_section.outputs {
                outputs = x.to_string();
            } else {
                outputs = String::from("NonReturning");
            }

            formatted.push_str(&format!(
                "
            FULL_CODE_SECTION_{}:
                types_section:
                    inputs: {},
                    outputs: {},
                    max_stack_height: {}
                code_section: {}",
                i,
                full_code_section.types_section.inputs,
                outputs,
                full_code_section.types_section.max_stack_height,
                code_section
            ));
        }

        formatted
    }
}

impl Default for FullCodeSection {
    fn default() -> Self {
        FullCodeSection {
            types_section: TypesSection {
                inputs: 0,
                outputs: None,
                max_stack_height: 0,
            },
            code_section: Vec::new(),
        }
    }
}

impl Display for FullCodeSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            full_code_section: {:?},
            ",
            self
        )
    }
}
