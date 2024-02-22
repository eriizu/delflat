# what's this?

A tool to copy files to a destination subdir with or without preserving path
components. When they are kept, they are inserted into the destination
filename, separated by dots.

example when not **keeping**:

```
"root/tata/src/stacks/create.c" becomes
"dest/tata/create.c"
```

example when **keeping**:

```
"root/tata/src/stacks/create.c" becomes
"dest/tata/src.stacks.create.c"
```

Basic usage:

```sh
cargo run -- --root=ROOT_DIR --dest=DEST_DIR [-k] FILES_TO_COPY
```

# use case

After cloning student repos for a project you would end up with something like this:

```
- delivery/
  - alice/
    - Makefile
    - src/
      - main.c
      - stack/
        - create.c
        - push.c
        - [...]
  - bob/
    - [something similar to alice]
  - [...]
```

(fig. 1)

That's usually not an issue. But then comes moss. A code plagiarism checking tool.
It expects a structure that looks more like this:

```
- flat_delivery/
  - alice/
    - main.c
    - stack.create.c
    - stack.push.c
    - [...]
  - bob/
    - main.c
    - strlen.c
    - [...]
```

(fig. 2)

This is excactly what delflat does.

The command to go from fig. 1 to fig. 2 is

```sh
delflat --root=delivery --dest=flat_delivery -k $(find delivery -name "*.c")
```

# Known issues

If the root directory starts with `./`, the files will also need to start with `./`,
even though they are referencing the same directory.
