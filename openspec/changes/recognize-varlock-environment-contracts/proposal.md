# Why

Arcantry can discover product intent, task queues and release history, but it cannot identify the environment contract that tells contributors and agents which runtime configuration exists without exposing its values. Projects using Varlock can therefore appear incomplete to Arcantry, while agents may inspect value-bearing `.env` files or create a competing configuration model instead of using the project's existing `.env.schema`.

# What changes

- Recognize shared and private `.env.schema` files as optional environment-contract sources with a versioned `env-spec@1` adapter.
- Keep environment contracts observation-only: inspection reports their identity, scope, visibility and compatibility without rendering file contents or resolving values.
- Allow an existing environment contract to be registered explicitly in Arcantry configuration without creating, editing, relocating or deleting the source file.
- Keep Varlock responsible for schema validation, value resolution, secret providers, injection, encryption and leak scanning.
- When an Arcantry operation itself needs environment values, preserve the process environment, prefer an available compatible Varlock setup, and fall back to dotenv-compatible `.env` loading when Varlock is absent or cannot be used.
- Keep optional Varlock detection, version incompatibility and resolution failures non-blocking unless the operation still lacks an input that its own contract requires after fallback.
- Explain how adopters and agents combine Arcantry discovery, best-effort environment loading and dedicated Varlock workflows without making Varlock a required dependency of Arcantry.

# Out of scope

- Installing, updating or embedding Varlock or its plugins, or requiring Varlock for Arcantry to start and perform environment-independent work.
- Invoking Varlock or reading environment value files during repository discovery, source validation or doctor diagnostics.
- Logging or serializing values obtained from the process environment, Varlock, `.env` files or secret providers.
- Creating a custom Arcantry environment format or reimplementing the `@env-spec` parser.
- Modifying an adopter's runtime, build, CI, hook or publication workflows to enable Varlock.
- Managing or rewriting `.env.schema` content.
