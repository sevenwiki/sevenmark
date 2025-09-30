# SevenMark Project Overview

## Purpose
SevenMark is a Domain Specific Language (DSL) parser designed for Sevenwiki - a sophisticated markup parser that handles diverse text formatting elements including text styles, block elements, tables, lists, media elements, and wiki-specific features.

## Tech Stack
- **Language**: Rust (Edition 2024, Rust 1.89.0+)
- **Primary Parser**: winnow 0.7.13 (parser combinator library with SIMD features)
- **Serialization**: serde 1.0.228 with derive features for JSON output
- **Error Handling**: anyhow 1.0.100 for comprehensive error management
- **Location Tracking**: line-span 0.1.5 for efficient line position calculation

## Optional Features
- **Server Mode** (default): axum 0.8.5, tokio 1.47.1, sea-orm 1.1.14 (PostgreSQL), utoipa/utoipa-swagger-ui, tracing
- **WASM Mode**: wasm-bindgen, js-sys, web-sys for web/bundler/nodejs targets

## Project Type
- Library crate (`lib.rs`) with multiple binary targets
- Supports WASM compilation (`cdylib`) and native Rust usage (`rlib`)

## Performance Characteristics
- SIMD-optimized parsing operations
- Zero-copy parsing where possible
- Typical speed: >10 MB/s on modern hardware
- Built-in performance measurement and reporting