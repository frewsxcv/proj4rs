# Changelog

<!--
All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).
-->

## Unreleased

### Added

* `Transform` implementations.
    - https://github.com/3liz/proj4rs/pull/6
    - Implement for a 2-tuple.
    - Implement for the `geo-types` geometries, them being placed behind a `geo-types` feature flag.

### Changed

* `Transform` trait signature.
    - https://github.com/3liz/proj4rs/pull/6
    - Alias `FnMut(f64, f64, f64) -> Result<(f64, f64, f64)>` behind a `TransformClosure`
    - `transform_coordinates()` takes a mutable reference to `f`, making it easier to layer `Transform` implementations.

## 0.1.1 - 2023-09-07

### Added

* Implement `Clone` on `Proj` type.
    - https://github.com/3liz/proj4rs/pull/2
* Added exemple in README
    - https://github.com/3liz/proj4rs/pull/3

