# Journal
## 20231016

Maybe parsing the glob in the program isn't usefull, as the shell can do it
for me if I don't use quote in my command.

That would eliminated the need for recursive directory exploration too, since
the arguments of the program would already contain all the files to process.

> Don't reinvent the wheel, if you can avoid it.

## 20231017

I'd like to make a function that takes a list of paths, and groups them by common roots.

If all paths share a common root, maybe abstract it.

Example 1:

- toto
    - a
        - riri
        - fifi
    - b
    - d
- tata
    - a
    - b
- titi

would regroup into

- toto:
    - toto/a/riri
    - toto/a/fifi
    - toto/b
    - toto/d
- tata:
    - tata/a
    - tata/b

## 20231019

Solution was to ask user the root path to use. All directories directly under
the root path are preserved but their subdirectories are flatenned so that you
end up with:

- root
    - dir1
        - all files from all subdirs of dir1
    - dir2
        - all files from all subdirs of dir2

Also to avoid duplicates, the names of the subdirs are inserted as a prefix
in the name of the preserved files.

`root/dir1/src/io/net.cpp` would become: `dest/dir1/src.io.net.cpp`

Also not using globs or recusrive dir walking at all, as other programmes can
do that very well for us.

Using a syntax such as `delflat source dest source/**/*.cpp` in a shell
produces perfect results.
