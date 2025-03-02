# Summary
- enter PR summary here
- what did you change
- why did you do it this way?
- Are there alternative ways we need to keep in mind?

## Testing (Read & Delete Me)
- Did you write tests to cover new or modified behavior?
- These are not necessary, but very welcome
- I am always happy to receive any PR's, but adding tests makes it easier for me to merge them!

## Conventional PR Titles (Read & Delete me)
- This crate uses `release-plz` to manage our deploys
- A CI check will verify your PR title to make sure it uses a valid conventional prefix like:
    - `feat: {title}` - corresponds to `Added` heading in changelog, backwards compatibile changes
    - `fix: {title}` - corresponds to `Fixed` heading in changelog, fixes bugs or bad behavior
    - `chore: {title}` - 
    - `docs: {title}`
    - `tests: {title}`
- Adding a `!` before the `:` will signify this is also a breaking change
    - This sort of change will cause a `MAJOR` version bump.
- You can also use scopes to further detail the area your PR is changing i.e.:
    - `feat(ladfile): add global types`
    - `fix(ladfile_builder): fix globals not getting exported`
    - `docs(bms): document weird vector behavior on MacOS`