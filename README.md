# HarfBuzz WASM PoC

This is a fun fork of the [llama.ttf](https://github.com/fuglede/llama.ttf).

## Prerequisites

[Install Rust:](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

[Install wasm-pack:](https://rustwasm.github.io/wasm-pack/installer/)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Install fonttools:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip3 install fonttools
```

Build HarfBuzz and WASM and add `libharfbuzz.so.0` and `libiwasm.so` to the `LD_PRELOAD`:

```bash
echo 'export LD_PRELOAD="$HOME/Projects/harfbuzz-9.0.0/build/src/libharfbuzz.so.0 $HOME/Projects/wasm-micro-runtime-WAMR-2.1.1/product-mini/platforms/linux/build/libiwasm.so"' >> ~/.bashrc
```

Build the `wasmpoc.ttf`:

```bash
make -C wasmpocttf/
```

Add the font and try it out using an editor of your choice.