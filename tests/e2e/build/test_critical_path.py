# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# pyre-strict


import typing
from dataclasses import dataclass

from buck2.tests.e2e_util.api.buck import Buck
from buck2.tests.e2e_util.buck_workspace import buck_test

from buck2.tests.e2e_util.helper.utils import filter_events


@dataclass
class critical_path_log:
    kind: str
    name: str
    category: str
    identifier: str
    execution_kind: str
    total_duration: str
    user_duration: str
    potential_improvement_duration: str


async def do_critical_path(buck: Buck, correct_analysis: bool) -> None:
    await buck.build("//:step_3", "--no-remote-cache")

    critical_path = (await buck.log("critical-path")).stdout.strip().splitlines()
    critical_path = [e.split("\t") for e in critical_path]

    trimmed_critical_path = [
        critical_path_log(e[0], e[1].split(" ")[0], e[2], e[3], e[4], e[5], e[6], e[7])
        for e in critical_path
    ]

    # There is now non-determism in this test since what we get back depends on
    # where the analysis becomes the longest path. This gets fixed later in
    # this stack.

    assert len(trimmed_critical_path) > 0
    expected = [
        ("load", "root//"),
        ("analysis", "root//:step_0"),
        ("analysis", "root//:step_1"),
        ("analysis", "root//:step_2"),
        ("analysis", "root//:step_3"),
        ("action", "root//:step_0"),
        ("action", "root//:step_1"),
        ("action", "root//:step_2"),
        ("action", "root//:step_3"),
        ("materialization", "root//:step_3"),
        ("compute-critical-path", ""),
    ]

    for s, e in zip(reversed(trimmed_critical_path), reversed(expected)):
        if s.kind == "action":
            assert s.execution_kind != ""
            assert s.execution_kind != "ACTION_EXECUTION_NOTSET"
        else:
            assert s.execution_kind == ""

        if not correct_analysis and s.kind == "analysis":
            break
        assert s.kind == e[0]
        assert s.name == e[1]


@buck_test(inplace=False)
async def test_critical_path(buck: Buck) -> None:
    await do_critical_path(buck, False)


@buck_test(inplace=False)
async def test_critical_path_longest_path_graph(buck: Buck) -> None:
    with open(buck.cwd / ".buckconfig", "a") as f:
        f.write("[buck2]\n")
        f.write("critical_path_backend2 = longest-path-graph\n")
    await do_critical_path(buck, True)


@buck_test(inplace=False)
async def test_critical_path_json(buck: Buck) -> None:
    import json

    await buck.build("//:step_3", "--no-remote-cache")
    critical_path = (
        (await buck.log("critical-path", "--format", "json"))
        .stdout.strip()
        .splitlines()
    )
    critical_path = [json.loads(e) for e in critical_path]

    assert len(critical_path) > 0
    expected = [
        ("load", "root//"),
        ("analysis", "root//:step_0"),
        ("analysis", "root//:step_1"),
        ("analysis", "root//:step_2"),
        ("analysis", "root//:step_3"),
        ("action", "root//:step_0"),
        ("action", "root//:step_1"),
        ("action", "root//:step_2"),
        ("action", "root//:step_3"),
        ("materialization", "root//:step_3"),
        ("compute-critical-path", None),
    ]

    for critical, exp in zip(reversed(critical_path), reversed(expected)):
        if critical["kind"] == "analysis":
            # There is now non-determism in this test since what we get back depends on
            # where the analysis becomes the longest path. This gets fixed later in
            # this stack.
            break

        assert "kind" in critical
        assert critical["kind"] == exp[0]

        if critical["kind"] == "compute-critical-path":
            assert "name" not in critical
        else:
            assert "name" in critical
            name = critical["name"].split(" ")[0]
            assert name == exp[1]

        if critical["kind"] == "action":
            assert "execution_kind" in critical
            assert critical["execution_kind"] != ""
            assert critical["execution_kind"] != "ACTION_EXECUTION_NOTSET"
        else:
            assert "execution_kind" not in critical


@buck_test(inplace=False)
async def test_critical_path_metadata(buck: Buck) -> None:
    await buck.build(
        "//:step_0",
        "--no-remote-cache",
        "-c",
        "client.id=myclient",
        "--oncall=myoncall",
    )

    build_graph_info = await filter_events(
        buck,
        "Event",
        "data",
        "Instant",
        "data",
        "BuildGraphInfo",
    )

    build_graph_info = build_graph_info[0]
    assert build_graph_info
    assert "username" in build_graph_info["metadata"]
    assert build_graph_info["metadata"]["client"] == "myclient"
    assert build_graph_info["metadata"]["oncall"] == "myoncall"


async def critical_path_helper(buck: Buck) -> typing.List[typing.Dict[str, typing.Any]]:
    critical_path_actions = await filter_events(
        buck,
        "Event",
        "data",
        "Instant",
        "data",
        "BuildGraphInfo",
        "critical_path2",
    )

    assert len(critical_path_actions) == 1
    return critical_path_actions[0]


@buck_test(inplace=False)
async def test_critical_path_execution_kind(buck: Buck) -> None:
    await buck.build("//:step_3", "--no-remote-cache")

    critical_path_actions = await critical_path_helper(buck)

    has_action_execution = False
    for action in critical_path_actions:
        assert action["entry"]
        # Every ActionExecution should have an execution kind and it shouldn't be 0 (default)
        if "ActionExecution" in action["entry"]:
            has_action_execution = True
            assert action["entry"]["ActionExecution"]["execution_kind"]
            assert action["entry"]["ActionExecution"]["execution_kind"] != 0

    # Should have at least 1 ActionExecution or something went wrong
    assert has_action_execution


@buck_test(inplace=False)
async def test_critical_path_rule_type(buck: Buck) -> None:
    await buck.build("//:step_0", "--no-remote-cache")

    critical_path_actions = await critical_path_helper(buck)

    for action in critical_path_actions:
        assert action["entry"]

        if "ActionExecution" in action["entry"]:
            assert action["entry"]["ActionExecution"]["target_rule_type_name"]
            assert (
                action["entry"]["ActionExecution"]["target_rule_type_name"] == "write"
            )


@buck_test(inplace=False)
async def test_critical_path_action_digest(buck: Buck) -> None:
    await buck.build("//:step_3", "--no-remote-cache")

    critical_path_actions = await critical_path_helper(buck)

    has_action_digest = False
    for action in critical_path_actions:
        assert action["entry"]
        if "ActionExecution" in action["entry"]:
            if "action_digest" in action["entry"]["ActionExecution"]:
                has_action_digest = True

    assert has_action_digest