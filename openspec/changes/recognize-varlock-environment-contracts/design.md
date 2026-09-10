# Approach

Extend the existing project-source model with an `environment-schema` kind and an Arcantry-versioned `env-spec@1` adapter. Default discovery uses stable `environment` and `environment-local` ids for `.env.schema` and `.local/.env.schema`. Configured paths continue to use the existing project-boundary and visibility rules.

The first adapter is observation-only. Arcantry may inspect filesystem metadata and hash the selected source when a drift-safe configuration plan requires it, but human and JSON output contain no schema contents, defaults, resolver references or resolved values. The adapter accepts only `ignore` and `observe` management. An explicit adoption plan may register an existing source as observed, but it cannot initialize a missing schema or write, relocate or delete the source.

Read-only repository discovery, source validation and doctor diagnostics do not execute Varlock. Separately, an Arcantry operation that explicitly needs environment values uses progressive loading: existing process values remain authoritative; an available, supported Varlock setup is attempted for missing values; and an unavailable executable, unsupported version, incompatible schema or failed resolution falls back to dotenv-compatible loading from the project-root `.env`. Failure of the optional Varlock path is diagnostic rather than fatal. The operation may fail only when its own contract requires a value that remains unavailable after fallback.

The Varlock boundary must return control to Arcantry on every compatibility or resolution failure. An eager integration that can terminate the host process before fallback, such as unconditional auto-loading, cannot implement this contract. Neither the Varlock path nor the dotenv fallback may log, serialize or include loaded values in diagnostics.

The implementation must extend the native Rust CLI and shared product contracts, including configuration schemas, adapter registries, standard-source discovery and serializable inspection output. The npm package remains a launcher for the native executable and does not own a parallel implementation. The change must also reconcile the supported-source inventory introduced by `add-deterministic-context-discovery` so the two changes do not create divergent discovery contracts.

# Trade-offs

Observation alone does not prove that a Varlock schema is valid or that a runtime environment can resolve it. It provides a useful and safe project-context boundary without making ordinary Arcantry inspection execute third-party code or access secrets. Best-effort runtime loading is a separate capability used only by operations that declare an environment-value dependency.

The dotenv-compatible fallback gives Arcantry a portable baseline when Varlock is missing or incompatible, but it does not reproduce Varlock validation, providers, environment-specific precedence or leak protection. Existing process values take precedence and fallback-loaded values only fill missing keys, keeping caller-provided configuration authoritative.

Calling the adapter `env-spec@1` keeps the source format independent from one implementation, while the documentation recommends Varlock as the current external owner. The adapter version describes Arcantry's compatibility contract and does not claim that the upstream `@env-spec` project has reached a matching major version.

Embedding an external format parser would give deeper schema inspection but would expand the dependency and security boundary beyond observation. Reimplementing the evolving format in Rust would transfer parser and security maintenance to Arcantry. Both remain excluded until a separate accepted change demonstrates a need that observation and external Varlock use cannot satisfy.
