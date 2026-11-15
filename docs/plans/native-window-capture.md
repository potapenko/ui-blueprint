# Native window capture delivery

Approved immediate execution: human01a1ddf8-8471-7e91-88d4-dc38e80cf501 and
01a1ddfa-0e60-74d2-b771-de13f9467652 in consumer01a1dcba; owner01a1db1e.
Mode Evolve, [NATIVE-CAPTURE@1](../specs/product/native-capture.md). Current master.
Reuses Native/Geometry/Identity/Privacy/CLI, D05 acquisition/image writer and
existing local release/install routes; screenshot-asset and QA governance read.

Write set: NativeWindowCapture.swift, existing HostHelper/CLI dispatch,
focused capture bridge tests, helper-source packaging list, native/CLI/spec routes,
existing macOS skill recipe, VERSION/CHANGELOG/local install notes. Shared owners
released these paths on2026-11-15. No donor/SDK/core geometry changes or new workers.
Use existing CaptureLifecycle and NativeArtifacts, add one installed capture-window
command and versioned metadata. Candidate0.3.0, no remote publication authority.

Checks: source-negative command tests, unchanged acquisition/mapping/writer cases,
CLI feature behavior, real exact SwiftUI and GPUI windows with existing permissions,
original PNG/data association, scoped non-author consumer review, matching local
bundle/skill installation and existing rollback checks. Reuse SDK acceptance.
All images system-temp and retained. Other temporary artifacts retire after use.
Status: implementation in progress; source candidates/tests not yet acceptance.
