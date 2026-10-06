# D05 core 0.1 decode working-bound feasibility

**Finite result: the remaining unproved terms are typed-prefix capacity/lifetime,
validation scratch, and now float_roundtrip numeric work.** No end-to-end numeric
upper reserve is certified. The concrete required pre-decode guard is specified
below, at the call to `serde_json::from_slice`, with counted quantities and an
explicit refusal condition. This is an actionable admission dependency, not a
claim that a B/depth/grammar upper bound is mathematically impossible.

The lower-bound family is retained as corroboration only; it does not answer the
host decision alone. In particular, it must not become a 64× multiplier. This
follow-up establishes that owned Content replay **moves** existing content rather
than recursively cloning whole trees, identifies the exact still-unproved capacity
terms, and qualifies the newly adopted production float_roundtrip branch.

## Authority, scope and pin

[Packet](../plans/ui-blueprint/packets/D05-decode-bound.md) at 8bb5c02 under approved
PLAN.UIB@1/ROADMAP D05. Reused full current Web/D05 closure; read registry 6,
D03@2 core 0.1 branch, D05@2/D05-MEMORY@1 and working-memory evidence. D02/D07,
CACHE/LIFECYCLE/EXCHANGE/MODEL/PRIVACY/BOUNDARIES/PERFORMANCE/ROADMAP dependencies,
RUST/DEV.RUST@2 remain the selected basis. Analysis 0.2 meaning/implementation is
excluded and must receive separate qualification. No source/spec/policy mutation.

Audited entry: `Document::from_json(&[u8], max_bytes)` followed by its
`validate_semantics` dispatch at saved core commit
`8bb5c02d70d3db91e5f5b07abd1402706ca156ce`. Used `git show` for these files, not
concurrently edited analysis-working-tree versions. Input bytes are caller-owned;
this entry tests length before parsing, but does not pre-admit token/container
counts, expected artifact kind or per-field capacities. Semantic limits run later.
`Document::validate()` is a different path: it first serializes an existing DTO,
then reparses/validates a second representation. It is not covered merely by
bounding an incoming slice; encoding/caller ownership needs a separate reservation.

SHA-256 of saved owners (reacquire with `git show <pin>:<path>`):

| Path | SHA-256 |
| --- | --- |
| Cargo.toml | d23f77e04234ad8b193aefc7bec235a52d942c29fa5383ae1ca4da400c91abe3 |
| Cargo.lock | 15823a7d64777cff89bc6c38d013a57100aeb6d346b55ff8e124442d1a736c49 |
| crates/schema/Cargo.toml | 7c650b5584ba326909a9c19b9a37e7c695cf6005ccc1fc5c9c20d3efb0981342 |
| crates/schema/src/lib.rs | 2342f672f5536261cc7612a866ca001a6da1bb74f903275a67a27ffb775e61aa |
| crates/schema/src/model.rs | 66e1b04aacdb398c37015f2792f5d20fa6de65bb5231040eca426c4f42725094 |
| crates/schema/src/validation.rs | 39e81311e1c7f19a2726a167c1d6b49f7086f35180f35bbc55c2daedd93914f5 |
| crates/schema/src/validation/graph.rs | 9592f5a4f2b2d85c47b00596ead613065446d9236b88aaabd361a2fc2e780afe |
| crates/schema/src/validation/outcomes.rs | ee88350704ec10f8b84f5b760256624512d5363a80516a99a5ba7f26e03b6c74 |

## Exact parser/features and source ledger

Original production normal/build feature closure was read with locked/offline `cargo tree`
(no build/profile): serde 1.0.229=alloc/default/derive/serde_derive/std;
serde_core 1.0.229=alloc/result/std; serde_json 1.0.151=alloc/default/std;
schemars 1.2.2=default/derive/schemars_derive/std. Manifests/lock matched the saved
pin. jsonschema's dev dependency explicitly adds `float_roundtrip`; do not reuse
that test/example graph as proof of production numeric-parser allocations.
No arbitrary_precision/raw_value/preserve_order/unbounded_depth in that closure.
The subsequent D07@3/ANALYSIS-FLOAT-001 change is now authoritative: runtime
serde_json features are **alloc/default/float_roundtrip/std**, verified by the same
locked/offline normal/build tree after reading D07@3. Active Cargo.toml SHA-256:
`1df267dd297e776644e380192198ab919b8fff1f6ae8383841e589ef77cc821b`;
Cargo.lock is unchanged at the hash above. The core 0.1 grammar/validation audit pin
remains 8bb5c02; this explicit feature overlay replaces the old default-only numeric
qualification. Concurrent analysis 0.2 types are still excluded. Another feature
change requires requalification, not reuse of this overlay by version number alone.

Exact cached crate archives matched Cargo.lock checksums; selected installed source
files were compared byte-for-byte with their archive members, not trusted by name:

| Package | Archive SHA-256 |
| --- | --- |
| serde 1.0.229 | 4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba |
| serde_core 1.0.229 | 67dca2c9c51e58a4791a4b1ed58308b39c64224d349a935ab5039aa360942a48 |
| serde_derive 1.0.229 | e7a5d71263a5a7d47b41f6b3f06ba276f10cc18b0931f1799f710578e2309348 |
| serde_json 1.0.151 | c841b55ecdae098c80dcae9cf767f6f8a0c2cdb3416bbef72181df4d0fe73f14 |

Primary owners inspected:

- [Serde commit 7fc3b4c](https://github.com/serde-rs/serde/tree/7fc3b4c30c94f73a96ebd1553f2b090d928fc3a8):
  serde/private/de.rs ContentVisitor/TaggedContentVisitor/ContentDeserializer;
  serde_core/private/content.rs and size_hint.rs, de/impls.rs Vec visitor;
  serde_derive/de/enum_adjacently.rs, enum_internally.rs and strict struct routing.
  Actual crate paths include `src/` under each package.
- [serde_json commit de850074](https://github.com/serde-rs/json/tree/de8500740cdcabffb9734f503e4889def823cf10):
  src/de.rs from_slice/from_trait, depth guard, SeqAccess/MapAccess and numeric
  feature branches; src/read.rs SliceRead strings/escapes; src/error.rs custom/
  invalid_type/make_error. No reader-mode or arbitrary-precision qualification.
- [Rust commit ac68faa /1.96.0](https://github.com/rust-lang/rust/tree/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96):
  alloc/src/raw_vec/mod.rs min_non_zero_cap/grow_amortized/finish_grow;
  alloc/src/alloc.rs Global grow; alloc/src/collections/btree/node.rs backing
  layouts. Library paths are under library/. Retrieved pinned source with 30s timeouts.

Only one permitted tiny layout diagnostic ran: cached serde 1.0.229 with Rust 1.96.0,
aarch64/8-byte pointer. `size_of<serde::__private229::de::Content>()=32`, pair=64,
String=24. It printed layouts only; no document parsing, adversarial input execution,
allocation profiling, benchmark or runtime collection. Private type use stays in
throwaway diagnostic code, never a shipping dependency. No crate installation.

## Counterexample derivation: valid JSON, invalid core value

Define `V_d` as d nested nonempty arrays around the empty string, and use:

```text
{"schema_version":"0.1.0","artifact":{"data":[V_d,...,V_d],"kind":"request"}}
```

There are k copies, k≥1. Fixed wrapper outside the data value is 64 bytes.
Each V_d is 2d+2 bytes; the outer data list is k(2d+3)+1 bytes. Consequently:

`encoded_length = 65 + k(2d+3)`.

Artifact is adjacently tagged. With data before kind, generated code invokes
ContentVisitor for the entire data value before selecting Request. The later
map-only Request parser rejects this array, but does not undo its allocation peak.
Putting kind first would reject this particular family earlier; accepted field
order is not restricted by current core 0.1. Internally tagged enums additionally
buffer their non-tag entries even when their tag appears early, so canonical order
alone is not a complete solution.

For d=120, JSON container depth is 123 (document, artifact, data-list, branch arrays),
below the default serde_json guard: remaining_depth starts 128 and rejects at zero.
Product graph-depth 8/9 is unrelated to this syntax buffering. SeqAccess has no size
hint; ContentVisitor starts an empty Vec then pushes each element. Rust 1.96's first
allocation for a 32-byte element reserves at least 4 slots. Each branch therefore
holds d live allocations of 4×32=128 bytes until variant selection. The outer list
holds at least 32k bytes of slots. Even ignoring all other allocations:

`simultaneously_live_Content_vector_layouts >= k(128d + 32) = 15,392k bytes`.

| Admitted byte ceiling B | k=floor((B−65)/243) | Actual encoded bytes | Source-derived lower bound, bytes |
| --- | ---: | ---: | ---: |
| 65,536 | 269 | 65,432 | 4,140,448 |
| 131,072 | 539 | 131,042 | 8,296,288 |
| 338,001 | 1,390 | 337,835 | 21,394,880 |
| 524,288 | 2,157 | 524,216 | 33,200,544 |

These are lower bounds on successfully requested live backing layouts before typed
rejection, not measured peaks and not upper reservations. Allocation failure/OOM
is not a structured decoder resource refusal. The family was **not executed**.
It demonstrates precisely why the existing ~2.6MB malformed 64KiB measurement is
not exhaustive. It does not establish a universal 64× upper multiplier.

## A valid local upper rule, not an end-to-end reservation

For the inspected push-grown Vec path, element size s>0, initial actual hint capacity
h and high-water initialized length n, a conservative backing-layout ceiling is
`s × max(h, 2n, m(s))`, where m(1)=8, m(2..1024)=4, m(>1024)=1. Use checked
arithmetic; n includes already consumed elements, not just an iterator's remaining
length. An empty Vec may allocate zero. This follows from RawVec's grow rule
and assumes Global reports the requested layout size, as pinned source does.
At moving growth, count both old and new backing layouts; using twice that local
ceiling is safe for those two layouts, but does not cover child element heaps.

For cautious typed preallocation, h is bounded by the supplied source cardinality
and floor(1MiB/s), **not** by the number of elements successfully decoded before a
failure. To turn this local rule into a working reservation one must bound/sum the
simultaneously live generic buffers, typed containers and their child allocations,
including malformed prefixes and validation clones. That aggregate bound is the
uncertified part; multiplying a measured DTO size does not establish it.

## Exact remaining factor: not an arbitrary Content-copy multiplier

`serde/private/de.rs:1485–1490` returns `self.content` from owned
ContentDeserializer::__deserialize_content_v1; `2433–2438` clones only in the **Ref**
variant. The pinned core ordinary derives have no untagged/flatten retry route
requiring that Ref replay. Do not multiply a complete Content tree by parser depth.
Internally tagged map replay can still retain the input map iterator backing while
TaggedContentVisitor allocates its replacement non-tag pair vector.

For one admitted input x, the required phase composition is:

`W(x) >= max(C_live + T_prefix + S_scratch + E_error + F_numeric, T_dto + V_scratch)`.

This is the **required proof obligation, not a proved numerical bound**. Input/frame
ownership is extra. The serde_json Deserializer/scratch drops before our semantic
validation begins; summing every phase as if simultaneous is unnecessary.

- **T_prefix remains unproved:** a live `Vec<T>` can initially reserve
  `h=min(source_remaining_count,floor(1MiB/sizeof(T)))` before its first element
  succeeds. Its source Content Vec/Map backing can remain alive. The missing ledger
  is the maximum simultaneous set of `(T,h,high_water_len)` and owned Box/string
  descendants over **all malformed prefixes**, not just valid completed nodes.
  Exact owners: serde_core/de/impls.rs VecVisitor plus core map-only Fields/enum
  seeds. Valid grammar cardinalities applied after decode do not provide h.
- **V_scratch remains unproved as an aggregate:** core graph pending/seen SourceKey
  clones, SessionContext/GoldenChain clones and Rust BTree collect paths coexist
  with the DTO. Rust 1.96 `btree/map.rs:2518–2532` first collects a Vec, stable-sorts
  it, then bulk-builds the tree; node layout alone omits that temporary Vec/sort
  workspace. This is the named remaining std collection path, not another profile.
- **F_numeric for the current feature is separately unqualified** as detailed below.

These are finite source-accounting questions. No measured coefficient or number
of successful graph nodes substitutes for their prospective maxima.

## Other allocations a real upper proof must account for

| Phase | Source fact / why the tempting shortcut is invalid |
| --- | --- |
| Generic/tagged buffer | Content recursively permits Seq/Map unrelated to the eventual core type. serde size_hint::cautious caps **initial preallocation per Vec** at 1MiB; Vec::push growth is not capped by it. Replayed Content exposes a known remaining count, so a typed Vec can preallocate before rejecting its first wrong element |
| Typed records | The selected owned core type graph is finite/non-recursive by value; graph children are SourceKeys. That does not constrain malformed Content trees. Vec<String>/properties/relations/text lists have no pre-decode element cap derived from 32/160 nodes. Id's 256-character check follows String allocation; unrestricted text is not an Id |
| Strings/escapes | SliceRead can borrow unescaped text for Content; owned DTO Strings still copy it. Escapes use a reusable scratch Vec and subsequent owned strings. Scratch length≤encoded string bytes is not capacity≤length, nor proof only one string representation is live |
| Capacity/grow overlap | RawVec requests max(2×old capacity, required, minimum 8/4/1 by element size), checked for layout overflow. Account old+new layouts if movement overlaps, plus still-live parent buffers. Global delegates same-alignment growth to realloc; its hidden allocator internals are not observable from these counts |
| Error path | serde_json custom errors format a String and box ErrorImpl/message before our map_err discards them. Payload-free returned ValidationError does not mean zero transient error allocation. Unknown fields/variants can format untrusted strings and static expected-name lists |
| Semantic validation | unique/context checks construct BTreeSets; graph validation builds maps, cloned SourceKey pending vectors/seen sets, and delta/source maps. GoldenChain additionally clones both snapshots and verification Transition for TransitionCase; SessionContext clones SessionDescriptor. The input DTO remains alive. BTree leaf/internal backing reserves 11 key/value slots and 12 child edges, not just live entries |

No complete sum of simultaneous generic/typed buffers, cautious preallocations,
error growth, validation temporaries and clone lifetimes is certified here. Merely
summing cumulative allocations also overstates sequentially dropped graph work and
would not be a useful peak proof. The named missing fact is an **all-input admission
and working-allocation envelope**, including malformed/tag-buffered syntax, before
calling from_json; more ordinary-case profiling does not supply it.

## Concrete guard required before typed decode

Guard location: after byte framing/admission, **before** `Document::from_json`
invokes `serde_json::from_slice`. Cover every external caller (including expected
request/channel artifacts); do not first decode `Value`/Content to perform the guard.
A preflight used for admission must itself have fixed bounded state and no
input-proportional buffers. No implementation or new limits are selected here.

A particularly small grammar-preserving check is that every core Artifact data
payload is an object: all pinned variants wrap map-only records. Rejecting an array
at that position before Content buffering removes the displayed family without
changing valid core 0.1 values. It is **not enough by itself**: malformed nested
record/vector fields and valid dense text lists remain. Do not promote that one
shape check into a complete memory proof.

The guard contract must carry/enforce this certificate over the exact admitted bytes:

| Quantity | Precise count / timing of refusal |
| --- | --- |
| D,Nv,Nm | JSON depth, all value tokens, all object entries, including duplicate/unknown/malformed-type prefixes; stop before exceeding admitted counters, not after typed rejection |
| n_i,m_j | Every array's element count and object's pair count; bound both aggregate slots and each container's arity because these become Content/typed Vec size hints |
| L_total,L_max | Aggregate and maximum string/key lexeme bytes (safe decoded-length overestimate); enforce while scanning raw bytes, before unescaping into scratch or formatting an error |
| NumLen,NumExp | Numeric lexeme/mantissa-digit and exponent magnitude budgets, including numbers in fields later rejected; scan without f64 conversion, or substitute a reviewed full numeric-path bound |
| Expected artifact | Admit only the caller's permitted artifact kind before expensive replay; locate it without buffering preceding data. This alone does not remove internal-tag buffering |
| R_work | A move-only working allowance covering all five parse terms and V_scratch above, retained until the corresponding allocations drop; existing retained/encoded quotas are not this allowance |

Concrete structural refusal: count every nonempty array/object and its slots while
scanning; fail before typed decode if D, Nv, Nm, any arity or string/number budget
exceeds its admitted value. For the displayed 64KiB family, the certificate exposes
32,281 nonempty arrays (not 32 product nodes), so an explicit container budget can
reject it without constructing the 4MiB Content backing. No value for that budget
is selected by this audit; legitimate platform samples must be admitted by policy.

For a counted container, apply the proved local RawVec rule R_s(n,h) above.
For the first generic pass, slot terms start with
`sum_arrays R_32(n_i,0) + sum_objects R_64(m_j,0)` plus strings/scratch and growth
overlap. Before invoking the decoder, reject if any admitted counter, checked
arithmetic or **complete** prospective charge exceeds R_work. Merely checking this
generic sum is insufficient: T_prefix, V_scratch, error growth and the current
numeric path must be qualified/enforced too. This explicitly prevents treating a
syntax census as an already proved whole decoder guard.

Minimum root/Integration decision: qualify these named allocation terms for the
permitted input grammar, or provide fallible charging/refusal at their real owners
before allocation. If using preflight limits, they must constrain malformed data
and be published as resource admission; silently demanding tag-first order changes
existing core 0.1 acceptance. A custom allocator returning null to infallible Vec
allocation can abort rather than return a bounded error, so it is not automatically
an enforcement solution. No parser/allocator/process architecture is adopted here.

## Current runtime float_roundtrip qualification

D07@3 now enables this feature for the same serde_json 1.0.151 archive. Root reported
204/2054 default-profile bit mismatches (example 0.9394596570041933), the feature-on
corpus passing and a 22-case production regression passing. These are **producer
reports**, not tests re-executed in this audit; they motivate fidelity and do not
supply a working-memory bound. Current features were independently read as above.

Additional exact source inspected and archive-matched: src/de.rs float_roundtrip
parse_long_integer/decimal/exponent, parse_decimal_overflow/f64_long_from_parts;
src/lexical/parse.rs, algorithm.rs, bhcomp.rs, bignum.rs, math.rs, num.rs, exponent.rs.

- Long numeric conversion stores integer/fraction digit data in scratch; clear()
  retains capacity. Scratch reconstruction in parse_decimal_overflow and later
  lexical parsing must be included, not just string-unescape scratch.
- Even concise numbers can reach `parse_concise_float → bhcomp`. Bigint::default
  reserves 20 limbs; small_atof holds real/theoretical Bigints together, and pow5/
  shift/multiply paths can grow limbs and allocate arithmetic temporaries.
- f64 MAX_DIGITS=769 bounds mantissa digits consumed by parse_mantissa, **not** the
  original numeric scratch, effective-exponent scaling, simultaneous limb capacity
  or multiplication workspace. The exact remaining F_numeric term is the peak
  of those buffers while the surrounding Content/typed prefix remains live.
- A complete qualification must bound effective exponent/limb growth through
  moderate/fallback/bhcomp and math.rs, or apply the explicit NumLen/NumExp admission
  guard before conversion. A field's final f64 size or prior std-only numeric term
  cannot stand in for it. Do not infer that huge raw exponents reach huge bignums;
  early overflow/underflow paths exist and must be included in that range proof.

The no-number lower-bound family and Serde/Rust layout facts survive this feature
change. The complete reservation does not inherit a default-only qualification.
No new profiling/fuzzing/runtime corpus was run and no Cargo/D07/source was changed
by this worker. Feature-on memory policy and analysis 0.2 qualification remain open.

Framing/input buffers/queues, replay/encoding, concurrent session/Completion owners,
retained quotas, helper/SDK/pixels, stack, allocator internals and RSS have separate
owners. This finite audit ends with the guard contract and three precisely named
unproved terms above, not an impossible-bound claim or an invented reserve.
