# LAD Preprocessor for mdbook

This is a preprocessor for `mdbook` that allows you to include `LAD` files in your markdown files.

## Usage

Add the following to your `book.toml`:

```toml
[preprocessor.lad_preprocessor]
```

Then any files with the `.lad.json` extension will be processed by the preprocessor.

So for example if you have the following structure:

```markdown
- [Normal file](normal_file.md)
- [LAD file](lad_file.lad.json)
```

The `lad_file.lad.json` will be processed by the preprocessor, and appropriate nested markdown will be generated from there on out using the `LAD file` chapter as the parent page.

If the file is not found

