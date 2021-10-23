# Getting Started

Kind of hellow word using [`workers-rs`](https://github.com/cloudflare/workers-rs) and Cloudflare KV.

Based on template for compiling Rust to WebAssembly and publishing the resulting worker to 
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Usage 

This worker starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting
your Worker. You can use only crates and modules that compile to the wasm32-unknown-unknown target. See Cargo.toml for example of Chrono crate (library) parameters.

You need to upload data to your KV store, code looks for "KV_JMENINY" store. There is example json file that can be uploaded to store.

```bash
# upload to preview store
wrangler kv:bulk put --binding=KV_JMENINY --preview jmeniny2.json
# upload to production
wrangler kv:bulk put --binding=KV_JMENINY jmeniny2.json
```

With `wrangler`, you can build, test, and deploy your Worker with the following commands: 

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

Read the latest `worker` crate documentation here: https://docs.rs/worker

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as 
compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and
modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple. 

Read more about this on the [`workers-rs` project README](https://github.com/cloudflare/workers-rs).

## Issues

If you have any problems with the `worker` crate, please open an issue on the upstream project 
issue tracker on the [`workers-rs` repository](https://github.com/cloudflare/workers-rs).

