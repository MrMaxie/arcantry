# Why

The native CLI is compared with a retired TypeScript implementation, while the documented and specified behavior is not itself executable. Both implementations can therefore agree on the same drift, and broad completeness claims can pass without executing the scenarios they name.

# What changes

- Establish one tracked inventory for every public command and its documented syntax.
- Make the compiled Rust binary pass independent black-box expectations for help, errors, exit status and filesystem effects.
- Replace text-presence checks and the TypeScript migration oracle with executable Rust evidence.
- Route public filesystem mutations through transactional execution with deterministic rollback evidence.

# Out of scope

- Changing the public command hierarchy or intended command behavior.
- Removing the public JavaScript library or its exports.
- Publishing, tagging or changing the product version.
- Adding concurrency to short, order-dependent filesystem transactions without benchmark evidence.
