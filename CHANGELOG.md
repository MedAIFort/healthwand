# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Ongoing development.

## [v0.2.0] - May 18, 2026

**M1 Complete: Repository Structure Migration**

### Added
- Minimal CI workflow (GitHub Actions)
- Pre-commit configuration for code quality
- Comprehensive regulatory mapping documentation (`regulatory-mapping.md`)
- PHI taxonomy and identifier documentation

### Changed
- Migrated from `phi-detector` to `healthwand` crate structure
- Reorganized repository: consolidated docs, configs, and tests
- Updated README with CLI examples and flag documentation

### Fixed
- UTF-8 character boundary handling in scanner context extraction
- Character encoding issues with multi-byte UTF-8 characters (em-dashes, etc.)

<!--
## [v0.1.0] - May 1, 2025
- Pre-release: Initial development
-->

<!--
## [v1.0.0] - Q3 2025
- Initial release: Core PHI detection, GitHub Action, and API server.
-->

