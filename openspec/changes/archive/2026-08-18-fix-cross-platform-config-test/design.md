# Approach

Build the test path with Node's platform-aware path resolver, then normalize separators only for TOML serialization. Assert against that constructed value so the test exercises the same absolute-path contract on Windows and Linux without changing production behavior.
