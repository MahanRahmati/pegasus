//! File operations and temporary file management module.
//!
//! This module provides centralized file system operations.
//! All file I/O operations are async and use Tokio for non-blocking execution.
//!
//! ## Submodules
//!
//! - [`operations`]: Core file system operations (read, write, delete, etc.)
//! - [`errors`]: Error types for file operations
//!
//! ## Features
//!
//! - Async file operations using Tokio
//! - XDG directory compliance helpers
//! - Comprehensive error handling with context

pub mod errors;
pub mod operations;
