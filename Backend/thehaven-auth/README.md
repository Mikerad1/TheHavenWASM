# thehaven_auth

This actor echoes messages received in a `SubMessage`. It is linked to the NATS
messaging provider with the contract `wasmcloud:messaging`.

For each incoming NATS message, the actor echoes the bytes back out to the
`reply_to` topic on the message

## How do I customize this template to use other contracts & interfaces?

- You can change what contracts this actor claims in `wasmcloud.toml` and the `Makefile`. In the future this will just be in `wasmcloud.toml`.
- You will then need to change the dependencies in `Cargo.toml` to import the interfaces for the contracts you want. Delete the `wasmcloud-interface-messaging` dep if you're not using that contract.
- Finally, change the `src/lib.rs` file, changing/deleting the current interface import and `impl` block, while adding a new import & `impl` for any contracts you added!

### Using the included Github Actions

The generated project include two GitHub actions: `build.yml` and `release.yml` under `.github/workflows`. The build action will automatically build, lint, and check formatting for your actor. The release action will automatically release a new version of your actor whenever code is pushed to `main`, or when you push a tag with the form `vX.Y.Z`.

These actions require 3 secrets

1. `WASH_ISSUER_KEY`, which can be generated with `wash keys gen issuer`, then look for the 58 character `Seed` value
1. `WASH_SUBJECT_KEY`, which can be generated with `wash keys gen module`, then look for the 58 character `Seed` value
1. `WASMCLOUD_PAT`, which can be created by following the [Github PAT instructions](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) and ensuring the `write:packages` permission is enabled
