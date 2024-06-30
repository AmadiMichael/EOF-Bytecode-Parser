use crate::constants::{
    CODE_SIZE_SIZE, CONTAINER_SIZE_SIZE, DATA_SIZE_SIZE, INPUTS_SIZE, KIND_SIZE, MAGIC_SIZE,
    MAX_NUM_CODE_SECTIONS, MAX_NUM_CONTAINER_SECTIONS, MAX_STACK_HEIGHT_SIZE,
    NUM_CODE_SECTIONS_SIZE, NUM_CONTAINER_SECTIONS_SIZE, OUTPUTS_SIZE, TERMINATOR_SIZE,
    TYPES_SIZE_SIZE, VERSION_SIZE,
};
use crate::types::{eof_container::EOFContainer, full_code_section::FullCodeSection};

pub fn parse_eof_bytecode(eof_bytecode: &[u8]) -> Result<EOFContainer, String> {
    let mut eof_container = EOFContainer::default();
    let mut offset = 0;

    // asserts its up to min header size
    if eof_bytecode.len() < 15 {
        return Err("Not up to min header size of 15 bytes".to_owned());
    }

    // assert it starts with 0xEF00
    if eof_bytecode[offset..offset + MAGIC_SIZE] != [0xEF, 0x00] {
        return Err("0xEF00 prefix not present".to_owned());
    }

    offset += MAGIC_SIZE;

    // asserts and set the version
    if eof_bytecode[offset] != 0x01 {
        return Err("Invalid EOF version".to_owned());
    }
    eof_container.header.version = eof_bytecode[offset];
    offset += VERSION_SIZE;

    // next should be kind types
    if eof_bytecode[offset] != 0x01 {
        return Err("Kind types expected next".to_owned());
    }
    offset += KIND_SIZE;

    // store types size
    eof_container.header.types_size =
        bytes2_to_u16(&eof_bytecode[offset..offset + TYPES_SIZE_SIZE]);
    offset += TYPES_SIZE_SIZE;

    // assert types size is divisible by 4
    if eof_container.header.types_size % 4 != 0 {
        return Err("Types size not divisible by 4".to_owned());
    }

    // for kind code
    if eof_bytecode[offset] != 0x02 {
        return Err("Kind code expected next".to_owned());
    }
    offset += KIND_SIZE;

    // store num code sections
    eof_container.header.num_code_sections =
        bytes2_to_u16(&eof_bytecode[offset..offset + NUM_CODE_SECTIONS_SIZE]);
    offset += NUM_CODE_SECTIONS_SIZE;

    // assert code section num corelates to types kind size
    if eof_container.header.num_code_sections * 4 != eof_container.header.types_size {
        return Err("Types size not equal to num code sections * 4".to_owned());
    }

    // assert 0 < code sections num <= 1024
    if !(eof_container.header.num_code_sections > 0
        && eof_container.header.num_code_sections <= MAX_NUM_CODE_SECTIONS as u16)
    {
        return Err("code sections num not > 0 and <= 1024".to_owned());
    }

    // store in code_size vector
    let mut i: u16 = 0;
    while i < eof_container.header.num_code_sections {
        let code_size = bytes2_to_u16(&eof_bytecode[offset..offset + CODE_SIZE_SIZE]);
        assert!(
            code_size > 0,
            "code size cannot be 0, found in section index {}",
            i
        );
        eof_container.header.code_sizes.push(code_size);

        offset += CODE_SIZE_SIZE;
        i += 1;
    }

    // if container is next, parse it too
    if eof_bytecode[offset] == 0x03 {
        // kind container
        offset += KIND_SIZE;

        // store num container sections
        eof_container.header.num_container_sections =
            bytes2_to_u16(&eof_bytecode[offset..offset + NUM_CONTAINER_SECTIONS_SIZE]);
        if eof_container.header.num_container_sections == 0 {
            return Err(
                "Cannot have 0 num of container section when kind container is present".to_owned(),
            );
        }

        offset += NUM_CONTAINER_SECTIONS_SIZE;

        // assert 0 < container sections num <= 256
        if !(eof_container.header.num_container_sections > 0
            && eof_container.header.num_container_sections <= MAX_NUM_CONTAINER_SECTIONS as u16)
        {
            return Err("container sections num not > 0 and <= 256".to_owned());
        }

        // store in container_size vector
        let mut i: u16 = 0;
        while i < eof_container.header.num_container_sections {
            let container_size = bytes2_to_u16(&eof_bytecode[offset..offset + CONTAINER_SIZE_SIZE]);
            if container_size == 0 {
                return Err(format!("size cannot be 0, found in section index {}", i));
            }

            eof_container.header.container_sizes.push(container_size);

            offset += CONTAINER_SIZE_SIZE;
            i += 1;
        }
    }

    // expect kind data next
    if eof_bytecode[offset] != 0x04 {
        return Err("Kind data expected next".to_owned());
    }
    offset += KIND_SIZE;

    // store data size
    eof_container.header.data_size = bytes2_to_u16(&eof_bytecode[offset..offset + 2]);
    offset += DATA_SIZE_SIZE;

    // assert terminator is next
    if eof_bytecode[offset] != 0x00 {
        return Err("Terminator expected next".to_owned());
    }
    offset += TERMINATOR_SIZE;

    // store types and code sections
    let mut i: usize = 0;
    let mut types_offset = offset;
    offset += eof_container.header.types_size as usize;
    while i < eof_container.header.num_code_sections as usize {
        // store types section
        // types_section.inputs
        let mut full_code_section = FullCodeSection::default();
        full_code_section.types_section.inputs = eof_bytecode[types_offset];
        types_offset += INPUTS_SIZE;

        // types_section.outputs
        full_code_section.types_section.outputs = {
            let outputs = eof_bytecode[types_offset];
            match outputs {
                128 => None,
                _ => Option::Some(outputs),
            }
        };
        types_offset += OUTPUTS_SIZE;

        // types_section.max_stack_height
        full_code_section.types_section.max_stack_height =
            bytes2_to_u16(&eof_bytecode[types_offset..types_offset + MAX_STACK_HEIGHT_SIZE]);
        types_offset += MAX_STACK_HEIGHT_SIZE;

        // store code section
        let s = &eof_bytecode.get(offset..offset + eof_container.header.code_sizes[i] as usize);
        if s.is_none() {
            return Err("insufficient bytecode length".to_owned());
        }
        full_code_section.code_section.extend_from_slice(s.unwrap());
        offset += eof_container.header.code_sizes[i] as usize;

        eof_container.full_code_section.push(full_code_section);

        i += 1;
    }

    if !(eof_container.full_code_section[0].types_section.inputs == 0
        && eof_container.full_code_section[0].types_section.outputs == None)
    {
        return Err("0th code section must have 0 inputs and 0 outputs".to_owned());
    }

    // store container section
    let mut i: usize = 0;
    while i < eof_container.header.num_container_sections as usize {
        eof_container.container_section.push(
            eof_bytecode[offset..offset + eof_container.header.container_sizes[i] as usize]
                .to_vec(),
        );

        offset += eof_container.header.container_sizes[i] as usize;
        i += 1;
    }

    // store data section
    if eof_container.header.data_size > 0 {
        let padding =
            (offset + eof_container.header.data_size as usize).saturating_sub(eof_bytecode.len());

        // if data size exceeds bytecode assume its a not-deployed container
        if padding == 0 {
            eof_container.data_section.extend_from_slice(
                &eof_bytecode[offset..offset + eof_container.header.data_size as usize],
            );
        } else {
            eof_container
                .data_section
                .extend_from_slice(&eof_bytecode[offset..]);
            let to_pad = vec![0; padding];
            eof_container.data_section.extend_from_slice(&to_pad);
        }

        offset += eof_container.header.data_size as usize;
    }

    // assert parsing stopped at end of eof bytecode or after it but never before the end
    // Should consider auxilary data ?
    if offset < eof_bytecode.len() {
        return Err("Trailing byte after parsing".to_owned());
    }

    post_validations(&eof_container, offset)?;

    Ok(eof_container)
}

fn post_validations(eof_container: &EOFContainer, len: usize) -> Result<(), String> {
    // for validating eof bytecode size against contents of the header based on the specs

    // - the total size of a deployed container without container sections must be 13 + 2*num_code_sections + types_size + code_size[0] + ... + code_size[num_code_sections-1] + data_size
    if eof_container.header.num_container_sections == 0 {
        let total_code_sizes: u16 = eof_container.header.code_sizes.iter().sum();

        // 13 here is for (2 byte magic + 1 byte version + 1 byte kind type + 2 byte kind length + 1 byte kind code  + 2 bytes num code sections + 1 byte kind data + 2 byte data size + 1 byte terminator)
        let expected = 13
            + (NUM_CODE_SECTIONS_SIZE as u16 * eof_container.header.num_code_sections)
            + eof_container.header.types_size
            + total_code_sizes
            + eof_container.header.data_size;
        if len as u16 != expected {
            return Err("Post condition 1 failed".to_owned());
        }
    } else {
        // - the total size of a deployed container with at least one container section must be 16 + 2*num_code_sections + types_size + code_size[0] + ... + code_size[num_code_sections-1] + data_size + 2*num_container_sections + container_size[0] + ... + container_size[num_container_sections-1]
        let total_code_size: u16 = eof_container.header.code_sizes.iter().sum();
        let total_container_size: u16 = eof_container.header.container_sizes.iter().sum();

        // 16 here is for (2 byte magic + 1 byte version + 1 byte kind type + 2 byte kind length + 1 byte kind code  + 2 bytes num code sections + 1 byte kind container  + 2 bytes num container sections + 1 byte kind data + 2 byte data size + 1 byte terminator)
        let expected = 16
            + (NUM_CODE_SECTIONS_SIZE as u16 * eof_container.header.num_code_sections)
            + eof_container.header.types_size
            + total_code_size
            + eof_container.header.data_size
            + (NUM_CONTAINER_SECTIONS_SIZE as u16 * eof_container.header.num_container_sections)
            + total_container_size;
        if len as u16 != expected {
            return Err("Post condition 2 failed".to_owned());
        }
    }

    Ok(())
}

fn bytes2_to_u16(bytes2: &[u8]) -> u16 {
    let hex = hex::encode(bytes2);
    return u16::from_str_radix(&hex, 16).unwrap();
}
