/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use async_trait::async_trait;
use buck2_cli_proto::new_generic::CompleteRequest;
use buck2_cli_proto::new_generic::CompleteResponse;
use buck2_cli_proto::new_generic::DebugEvalRequest;
use buck2_cli_proto::new_generic::DebugEvalResponse;
use buck2_cli_proto::new_generic::ExpandExternalCellRequest;
use buck2_cli_proto::new_generic::ExpandExternalCellResponse;
use buck2_cli_proto::new_generic::ExplainRequest;
use buck2_cli_proto::new_generic::ExplainResponse;
use buck2_server_ctx::ctx::ServerCommandContextTrait;
use buck2_server_ctx::other_server_commands::OtherServerCommands;
use buck2_server_ctx::other_server_commands::OTHER_SERVER_COMMANDS;
use buck2_server_ctx::partial_result_dispatcher::NoPartialResult;
use buck2_server_ctx::partial_result_dispatcher::PartialResultDispatcher;

use crate::commands::build::build_command;
use crate::commands::complete::complete_command;
use crate::commands::ctargets::configured_targets_command;
use crate::commands::debug_eval::debug_eval_command;
use crate::commands::expand_external_cell::expand_external_cell_command;
use crate::commands::explain::explain_command;
use crate::commands::install::install_command;
use crate::commands::query::aquery::aquery_command;
use crate::commands::query::cquery::cquery_command;
use crate::commands::query::uquery::uquery_command;
use crate::commands::targets::targets_command;
use crate::commands::targets_show_outputs::targets_show_outputs_command;

struct OtherServerCommandsInstance;

#[async_trait]
impl OtherServerCommands for OtherServerCommandsInstance {
    async fn build(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: buck2_cli_proto::BuildRequest,
    ) -> anyhow::Result<buck2_cli_proto::BuildResponse> {
        build_command(ctx, partial_result_dispatcher, req).await
    }
    async fn install(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: buck2_cli_proto::InstallRequest,
    ) -> anyhow::Result<buck2_cli_proto::InstallResponse> {
        install_command(ctx, partial_result_dispatcher, req).await
    }
    async fn uquery(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
        req: buck2_cli_proto::UqueryRequest,
    ) -> anyhow::Result<buck2_cli_proto::UqueryResponse> {
        uquery_command(ctx, partial_result_dispatcher, req).await
    }
    async fn cquery(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
        req: buck2_cli_proto::CqueryRequest,
    ) -> anyhow::Result<buck2_cli_proto::CqueryResponse> {
        cquery_command(ctx, partial_result_dispatcher, req).await
    }
    async fn aquery(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
        req: buck2_cli_proto::AqueryRequest,
    ) -> anyhow::Result<buck2_cli_proto::AqueryResponse> {
        aquery_command(ctx, partial_result_dispatcher, req).await
    }
    async fn targets(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
        req: buck2_cli_proto::TargetsRequest,
    ) -> anyhow::Result<buck2_cli_proto::TargetsResponse> {
        targets_command(ctx, partial_result_dispatcher, req).await
    }
    async fn targets_show_outputs(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: buck2_cli_proto::TargetsRequest,
    ) -> anyhow::Result<buck2_cli_proto::TargetsShowOutputsResponse> {
        targets_show_outputs_command(ctx, partial_result_dispatcher, req).await
    }
    async fn ctargets(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: buck2_cli_proto::ConfiguredTargetsRequest,
    ) -> anyhow::Result<buck2_cli_proto::ConfiguredTargetsResponse> {
        configured_targets_command(ctx, partial_result_dispatcher, req).await
    }

    async fn complete(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: CompleteRequest,
    ) -> anyhow::Result<CompleteResponse> {
        complete_command(ctx, partial_result_dispatcher, req).await
    }

    async fn debug_eval(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        req: DebugEvalRequest,
    ) -> anyhow::Result<DebugEvalResponse> {
        debug_eval_command(ctx, req).await
    }

    async fn explain(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: ExplainRequest,
    ) -> anyhow::Result<ExplainResponse> {
        explain_command(ctx, partial_result_dispatcher, req).await
    }

    async fn expand_external_cell(
        &self,
        ctx: &dyn ServerCommandContextTrait,
        partial_result_dispatcher: PartialResultDispatcher<NoPartialResult>,
        req: ExpandExternalCellRequest,
    ) -> anyhow::Result<ExpandExternalCellResponse> {
        expand_external_cell_command(ctx, partial_result_dispatcher, req).await
    }
}

pub(crate) fn init_other_server_commands() {
    OTHER_SERVER_COMMANDS.init(&OtherServerCommandsInstance);
}
