# nscriptsplit
Quick tool to split `nscript.txt` files.
This was made for `kirikiri-web` to assist in converting the script files.

## Installation
Clone this repository and run `cargo build --release` then get the binary in the `target` directory (you may need to search for your platform)

## Usage
Run the binary with the nscript file as argument, it will then create the directories and files.

Output directory should look like this:
```
script
├── F-Block
└── S-Block
```

`F-Block` and `S-Block` contain the F-Blocks and S-Blocks respectively
