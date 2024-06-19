use constants::{
    CODE_SIZE_SIZE, CONTAINER_SIZE_SIZE, DATA_SIZE_SIZE, INPUTS_SIZE, KIND_SIZE, MAGIC_SIZE,
    MAX_NUM_CODE_SECTIONS, MAX_NUM_CONTAINER_SECTIONS, MAX_STACK_HEIGHT_SIZE,
    NUM_CODE_SECTIONS_SIZE, NUM_CONTAINER_SECTIONS_SIZE, OUTPUTS_SIZE, TERMINATOR_SIZE,
    TYPES_SIZE_SIZE, VERSION_SIZE,
};
use std::process::exit;
use types::{eof_container::EOFContainer, full_code_section::FullCodeSection};
mod constants;
mod types;

fn main() {
    // Some Examples can be found in ../inputs.json
    let input = "0xef000101000404000202000100060000800000aabb6000e0000000";

    let mut input = input.replace(" ", "").to_lowercase();
    if input.starts_with("0x") {
        input = input.strip_prefix("0x").unwrap().to_owned();
    }

    let bytecode = hex::decode(input).unwrap_or_else(|x| {
        println!("Err: {}", x);
        exit(0);
    });
    _main(&bytecode);
}

fn _main(bytecode: &[u8]) {
    let eof_container = parse_eof_bytecode(&bytecode);
    println!("{}", eof_container);
}

fn parse_eof_bytecode(eof_bytecode: &[u8]) -> EOFContainer {
    let _ = eof_bytecode;

    let mut eof_container = EOFContainer::default();
    let mut offset = 0;

    // asserts its up to min header size
    assert!(
        eof_bytecode.len() >= 15,
        "Not up to min header size of 15 bytes",
    );

    // assert it starts with 0xEF00
    assert!(
        eof_bytecode[offset..offset + MAGIC_SIZE] == [0xEF, 0x00],
        "0xEF00 prefix not present"
    );
    offset += MAGIC_SIZE;

    // asserts and set the version
    assert!(eof_bytecode[offset] == 0x01, "Invalid EOF version");
    eof_container.header.version = eof_bytecode[offset];
    offset += VERSION_SIZE;

    // next should be kind types
    assert!(eof_bytecode[offset] == 0x01, "Kind types expected next");
    offset += KIND_SIZE;

    // store types size
    eof_container.header.types_size =
        bytes2_to_u16(&eof_bytecode[offset..offset + TYPES_SIZE_SIZE]);
    offset += TYPES_SIZE_SIZE;

    // assert types size is divisible by 4
    assert!(
        eof_container.header.types_size % 4 == 0,
        "Types size not divisible by 4"
    );

    // for kind code
    assert!(eof_bytecode[offset] == 0x02, "Kind code expected next");
    offset += KIND_SIZE;

    // store num code sections
    eof_container.header.num_code_sections =
        bytes2_to_u16(&eof_bytecode[offset..offset + NUM_CODE_SECTIONS_SIZE]);
    offset += NUM_CODE_SECTIONS_SIZE;

    // assert code section num corelates to types kind size
    assert!(
        eof_container.header.num_code_sections * 4 == eof_container.header.types_size,
        "Types size not equal to num code sections * 4"
    );

    // assert 0 < code sections num <= 1024
    assert!(
        eof_container.header.num_code_sections > 0
            && eof_container.header.num_code_sections <= MAX_NUM_CODE_SECTIONS as u16,
        "code sections num not > 0 and <= 1024"
    );

    // store in code_size vector
    let mut i: u16 = 0;
    loop {
        if i == eof_container.header.num_code_sections {
            break;
        }

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
        assert!(
            eof_container.header.num_container_sections > 0,
            "Cannot have 0 num of container section when kind container is present"
        );
        offset += NUM_CONTAINER_SECTIONS_SIZE;

        // assert 0 < container sections num <= 256
        assert!(
            eof_container.header.num_container_sections > 0
                && eof_container.header.num_container_sections <= MAX_NUM_CONTAINER_SECTIONS as u16,
            "container sections num not > 0 and <= 256"
        );

        // store in container_size vector
        let mut i: u16 = 0;
        loop {
            if i == eof_container.header.num_container_sections {
                break;
            }

            let container_size = bytes2_to_u16(&eof_bytecode[offset..offset + CONTAINER_SIZE_SIZE]);
            assert!(
                container_size != 0,
                " size cannot be 0, found in section index {}",
                i
            );

            eof_container.header.container_sizes.push(container_size);

            offset += CONTAINER_SIZE_SIZE;
            i += 1;
        }
    }

    // expect kind data next
    assert!(eof_bytecode[offset] == 0x04, "Kind data expected next");
    offset += KIND_SIZE;

    // store data size
    eof_container.header.data_size = bytes2_to_u16(&eof_bytecode[offset..offset + 2]);
    offset += DATA_SIZE_SIZE;

    // assert terminator is next
    assert!(eof_bytecode[offset] == 0x00, "Terminator expected next");
    offset += TERMINATOR_SIZE;

    // store types and code sections
    let mut i: usize = 0;
    let mut types_offset = offset;
    offset += eof_container.header.types_size as usize;
    loop {
        if i == eof_container.header.num_code_sections as usize {
            break;
        }

        // store types section
        // types_section.inputs
        let mut full_code_section = FullCodeSection::default();
        full_code_section.types_section.inputs = eof_bytecode[types_offset];
        types_offset += INPUTS_SIZE;

        // types_section.outputs
        full_code_section.types_section.outputs = {
            let r = eof_bytecode[types_offset];
            if r == 128 {
                None
            } else {
                Option::Some(r)
            }
        };
        types_offset += OUTPUTS_SIZE;

        // types_section.max_stack_height
        full_code_section.types_section.max_stack_height =
            bytes2_to_u16(&eof_bytecode[types_offset..types_offset + MAX_STACK_HEIGHT_SIZE]);
        types_offset += MAX_STACK_HEIGHT_SIZE;

        // store code section
        full_code_section.code_section.extend_from_slice(
            &eof_bytecode[offset..offset + eof_container.header.code_sizes[i] as usize],
        );
        offset += eof_container.header.code_sizes[i] as usize;

        eof_container.full_code_section.push(full_code_section);

        i += 1;
    }

    assert!(
        eof_container.full_code_section[0].types_section.inputs == 0
            && eof_container.full_code_section[0].types_section.outputs == None,
        "0th code section must have 0 inputs and 0 outputs"
    );

    // store container section
    let mut i: usize = 0;
    loop {
        if i == eof_container.header.num_container_sections as usize {
            break;
        }

        eof_container.container_section.push(
            eof_bytecode[offset..offset + eof_container.header.container_sizes[i] as usize]
                .to_vec(),
        );

        offset += eof_container.header.container_sizes[i] as usize;
        i += 1;
    }

    // store data section
    if eof_container.header.data_size > 0 {
        assert!(
            (offset + eof_container.header.data_size as usize) <= eof_bytecode.len(),
            "data section read overflowed eof bytecode"
        );
        eof_container.data_section.extend_from_slice(
            &eof_bytecode[offset..offset + eof_container.header.data_size as usize],
        );
        offset += eof_container.header.data_size as usize;
    }

    // assert parsing stopped at end of eof bytecode
    // Should consider auxilary data ?
    assert!(offset == eof_bytecode.len(), "Trailing byte after parsing");

    post_validations(&eof_container, eof_bytecode);

    eof_container
}

fn post_validations(eof_container: &EOFContainer, eof_bytecode: &[u8]) {
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
        assert!(
            eof_bytecode.len() as u16 == expected,
            "Post condition 1 failed"
        );
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
        assert!(
            eof_bytecode.len() as u16 == expected,
            "Post condition 2 failed"
        );
    }
}

fn bytes2_to_u16(bytes2: &[u8]) -> u16 {
    let hex = hex::encode(bytes2);
    return u16::from_str_radix(&hex, 16).unwrap();
}
