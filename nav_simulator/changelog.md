
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.0.3] - 2026-05-21
### Changed
- Updated the vehicle module to use waypoints for guidance instead of just a single goal location

## [0.0.2] - 2026-05-21
### Changed
- Updated `util.rs` to include a telemetry enum and implemented a Populate trait to populate the telemetry from its variants
- Update `main.rs` to obtain the telemetry data from a thread (via a channel) and print it
 
## [0.0.1] - 2026-05-21
### Changed
- Removed notes.md file