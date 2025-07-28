# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] 28.07.2025
### Added
- Ability to enter latitude and longitude coordinates
- Fetch weather data with PirateWeather API asynchronously
- Widget toggle View
- Temperature forecast graphical widget
- Sun and moon graphical widget
- Current weather widget
- Log View populated asynchronously
- Support to build both native desktop and web
- GHA for unit tests, web deploy

---

## Release process

1. Update this `CHANGELOG.md` with a new entry.
2. Commit the changes:
   ```bash
   git commit -am "Release vX.Y.Z"
   ```
3. Tag the release:
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```
4. Create a GH release.
5. Bump the version in `Cargo.toml`.
