# MD Content Generator

Do you! like me! Take notes in markdown and your notes end up very long?
This tool was created for this exact reason, run the program and pass a markdown
file as the argument. This program will generate a contents page for you to copy!

This is based on using # as levels for headings!

## Useage:

```zsh
cd md-content-generator
cargo run ./path/to/markdown/file.md < contents.txt 
```
1. Run the program from the directory
2. Pass a markdown file path as an argument
3. redirect into a text file. 

> Note: You can use md or mdx files.


## Example Output:

```
Opening C:\Users\User\Documents\Dev\Projects\personal-site\New Astro Version\src\pages\notes\Clean-Code-with-Uncle-Bob.mdx
<details>
<summary><span style='font-weight:bold; font-size: 1.4rem;'>Contents</span></summary>
  - [Speed and efficiency](#speed-and-efficiency)
  - [One Thing](#one-thing)
  - [Open-Close Principle](#open-close-principle)
    - [Side Effects ](#side-effects)
      - [Command Query Separation ](#command-query-separation)
</details>
```

The first line is a debug output telling you what file you opened, remember to remove it before you paste 
this into a markdown file.

## Live Example:
https://merichard123.github.io/notes/The-Rust-Language/

## See an Issue?
I'll be honest I wouldn't be surprised but feel free to open an issue or even a pr if you feel like it. 
