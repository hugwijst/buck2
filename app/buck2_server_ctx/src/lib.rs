/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

#![feature(let_chains)]
#![feature(error_generic_member_access)]
#![feature(used_with_arg)]

pub mod bxl;
pub mod command_end;
pub mod concurrency;
pub mod ctx;
pub mod global_cfg_options;
pub mod logging;
pub mod other_server_commands;
pub mod partial_result_dispatcher;
pub mod pattern;
pub mod pattern_parse_and_resolve;
pub mod stderr_output_guard;
pub mod stdout_partial_output;
pub mod streaming_request_handler;
pub mod target_resolution_config;
pub mod template;
pub mod test_command;
