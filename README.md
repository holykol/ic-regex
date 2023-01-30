# ic-regex

[![Rust](https://github.com/holykol/ic-regex/actions/workflows/build.yml/badge.svg)](https://github.com/holykol/ic-regex/actions/workflows/build.yml)

Welcome to `ic-regex`, a canister that exposes API of [`regex`](https://docs.rs/regex/latest/regex/) crate for your pattern matching needs on IC. It is intended to serve as a replacement until Motoko language adds a native regular expression library.

## Using from Motoko
See [`./src/example/main.mo`](./src/example/main.mo) for a complete example.

## Considerations

**Performance:**
* Try to use `precompile` method to cache regexes that are going to be used a lot.
* Always prefer to use `*_batch` methods, as queries to IC will take more time compared to actual matching.

**Limits:**
* There is a 2MB call size limit, you will have to try very hard to reach it.
* The only thing more difficult will be to exhaust request cycles limit.
* So unless you are doing gigabytes of data parsing on IC, you will likely be fine.

**Cost:**
* Napkin math suggests that checking a valid email (as shown in the Motoko example) 10_000 times would cost just under 3 cents. While this might add up to a pretty penny, it's still better than not being able to use regex on chain at all.
* Consider [opting out of Unicode support](https://docs.rs/regex/latest/regex/#opt-out-of-unicode-support) not so much for performance, but to save cycles

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# needed for ic-kit to work
git submodule update --init

# install deps
npm i

# Starts the replica, running in the background
dfx start --background

# Generates bindings
dfx generate

# Deploys your canisters to the replica
dfx deploy

# run benchmarks
export REGEX_CANISTER_ID=$(cat ./.dfx/local/canister_ids.json | jq -r .regex.local)
npm run bench
```

Then visit Candid interface link last command gave you to try out available methods!

## Deployed version
Feel free to try publicly deployed version: **`2looq-saaaa-aaaak-qbv7a-cai`** ([Open in Candid UI](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/?id=2looq-saaaa-aaaak-qbv7a-cai))

Remember to donate some cycles to keep the сanister running for everybody:
```bash
# This command will send 1T cycles to the canister. Run from repo root
dfx canister --network ic deposit-cycles 1000000000000 regex
```

## Deploying yourself
https://internetcomputer.org/docs/current/developer-docs/deploy/deploying-and-upgrading

## Useful links

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Development Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)


## License
MIT