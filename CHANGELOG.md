# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
### Added
- README, LICENSE, .gitignore
- Cargo.toml
- skeleton of src/ (main, lib, app)

---

## Release process

1. Update the version in `Cargo.toml`.
2. Update this `CHANGELOG.md` with a new entry.
3. Commit the changes:
   ```bash
   git commit -am "Release vX.Y.Z"
   ```
4. Tag the release:
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```
5. (Optional) Create a [GitHub Release](https://github.com/your-username/your-repo/releases) with notes.
