<div align="center">

# minils

minils is a reimplementation of _exa_ from the ground up.

**README Sections:** [Options](#options) — [Installation](#installation)

---

**minils** was created as a personal project inspired by The Rust Programming Language by 
Steve Klabnik and Carol Nichols. 
</div>

---

<a id="options">
<h1>Command-line options</h1>
</a>

List directory contents.
Ignore files and directories starting with a '.' by default

### Display options

- **-1**, **--oneline**: display one entry per line
- **-G**, **--grid**: display entries as a grid (default)
- **-l**, **--long**: display extended details and attributes

### Filtering options

- **-a**, **--all**: show hidden and 'dot' files
- **-d**, **--list-dirs**: list directories like regular files
- **-D**, **--only-dirs**: list only directories
- **-f**, **--only-files**: list only files


---

<a id="installation">
<h1>Installation</h1>
</a>

minils is available for unix systems: MacOs and Linux.

### Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:

    cargo install minils
