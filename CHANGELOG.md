This file contains the changelog for the JXLConvert project. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- Added support for passing source comics directory as a command line argument.
- Added suuport for specifying the output directory for the converted comics. (#11)
   - Also introduces a new cli argument --destination.
- Bumped dependencies to latest versions.

## [0.1.1] - 2024-12-29

Published the initial release of the JXLConvert project to crates.io.

## [0.1.0] - 2024-12-29

This is the initial release of the JXLConvert project. It includes the following features:

- Basic CLI interface.
   - Prompt user for directory containing comic archives.
   - Convert images in comic archives to JPEG XL format.
      - Currently only supports archives in `.cbz` format.
      - And images in `.jpg` and `.png` format.