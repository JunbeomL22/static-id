# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [0.2.2] - 2024-09-26
* added structs: 
  * StaticId16x0
  * StaticId32x0
  * StaticId64x0
  
## [0.2.0] - 2024-09-08
* added structs: 
  * StaticId16x16
  * StaticId16x32
  * StaticId16x64
  * StaticId32x16
  * StaticId32x32
  * StaticId32x64
  * StaticId64x16
  * StaticId64x32
  * StaticId64x64
* changed: 
  * StaticId: StaticId32x16 => StaticId32x32 (16 is somewhat small)

## [0.1.4] - 2024-09-01
- Display implementation
- code_str and venue_str

## [0.1.4] - 2024-09-01
- Defualt method
- Hasher test in multi-threading environment

## [0.1.2] - 2024-09-01
- benchmark for comparison between an existing and non-existing element creation
- unittest for multi-threading environment

## [0.1.1] - 2024-09-01

- CHANGELOG.md file to track changes to the project.
- Modified the debug format of `StaticId` for improved readability and debugging experience.

## [0.1.0] - 2024-09-01
### Initial commit