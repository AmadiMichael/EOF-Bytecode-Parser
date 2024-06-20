# EOF Bytecode Parser

EOF Bytecode Parser with Header Validation.

## How to run

```zsh
cargo run -- [bytecode | filepath]
```

e.g

```zsh
cargo run -- bytecode.txt
```

```zsh
cargo run -- 0xef0001010004020001000604000200008000016000e0000000aabb
```

When trying parsing bytecode that's spaced for readability e.g `EF00 01 01 0004 02 0001 0008 03 0001 0030 04 0000 00 00 80 0004 5F 5F 60FF 5F EC00 00 EF00 01 01 0004 02 0001 0004 03 0001 0014 04 0000 00 00 80 0002 5F 5F EE00 EF00 01 01 0004 02 0001 0001 04 0000 00 00 80 0000 00`, it can be done via a file or from the terminal input by putting it in string quotes like so:

```zsh
cargo run -- ""EF00 01 01 0004 02 0001 0008 03 0001 0030 04 0000 00 00 80 0004 5F 5F 60FF 5F EC00 00 EF00 01 01 0004 02 0001 0004 03 0001 0014 04 0000 00 00 80 0002 5F 5F EE00 EF00 01 01 0004 02 0001 0001 04 0000 00 00 80 0000 00"
```

Inputs of file (contents of the file) or bytecode can also include or omit `0x` prefix and can be upper or lower cased. Only requirement is that it be a valid hexadecimal value.
