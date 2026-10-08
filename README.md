# UI Blueprint

Local Rust tools for inspecting saved UI observations, measuring geometry,
checking explicit expectations and preparing model-free engineering documents.
Selected Web and macOS modules use matching owned worker/helper executables.

To build a local bundle into an existing directory:

```sh
BUNDLE="$(mktemp -d "${TMPDIR:-/tmp}/uib-local.XXXXXX")"
python3 distribution.py build --modules combined --destination "$BUNDLE"
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
I02 verifies all four selections at product source `94724df`, including the Native
session entry point and saved-data compare export. See the
[I02 qualification receipt](docs/plans/ui-blueprint/receipts/I02-current-distribution.md)
for exact pins and limits. Integrated pilots and release acceptance remain separate.
