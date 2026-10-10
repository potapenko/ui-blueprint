# UI Blueprint

Local Rust tools for inspecting saved UI observations, measuring geometry,
checking explicit expectations and preparing model-free engineering documents.
Selected Web and macOS modules use matching owned worker/helper executables.

To build a local bundle into an existing directory:

```sh
BUNDLE="$(mktemp -d "${TMPDIR:-/tmp}/uib-local.XXXXXX")"
python3 distribution.py build --modules combined --revision a0265843634fce8bc6a942fc1f276391a51c54c6 --destination "$BUNDLE"
python3 "$BUNDLE/distribution.py" verify --destination "$BUNDLE"
"$BUNDLE/uiblueprint" --help
```

Choose `core`, `web`, `native` or `combined`. Requires an already prepared pinned
Rust toolchain/cache; Native adds Swift/Xcode. Current tested host: arm64 macOS
27.0.1. Nothing is installed in PATH and no app/browser/model is launched.

The [distribution guide](docs/development/distribution.md) contains prerequisites,
examples, artifact layout, limitations and safe removal/recovery. See
[CLI details](docs/development/cli.md), [dependency notices](THIRD_PARTY_NOTICES.md)
and the [specification registry](docs/specs/README.md).
I02 qualifies all four installed selections at product source `a026584`, including
E05's complete literal real-form prompts; Native shipping source remains `e641543`.
See the [I02 qualification receipt](docs/plans/ui-blueprint/receipts/I02-current-distribution.md)
for exact pins, fresh checks and reused evidence. Director/Settings prompts can be
submitted directly, but their generated images remain failed/unverified/draft and
may omit or misbind facts. Compilation stays model-free. Q03 bounded usefulness is
completed. [Final Q01 acceptance](docs/plans/ui-blueprint/receipts/Q01-integrated-acceptance.md#current-native-final-reconciliation--bounded-acceptance-2026-10-10)
accepts bounded Native correctness/source fidelity and separate shipping numeric
evidence with limited availability: 236 observed / 6 failed AX requests out of 242.
Those target_unresolved failures and diagnostic quality=false remain; capture can
succeed independently. The cause is unknown. Recovery is a new explicit request,
never automatic retry. Overall P7/release reconciliation remains with root.
