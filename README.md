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

Clone and build wasm libary:
```bash
git clone https://github.com/bytecodealliance/wasm-micro-runtime.git

```
Follow this [guide](https://github.com/bytecodealliance/wasm-micro-runtime/blob/main/product-mini/README.md) to build the WASM libary.

Run the follow build at the **root** of the wasm-micro-runtime project. It is **important** that these commmands are ran at the root of the project for example ` ~/wasm-micro-runtime `. 
```bash
cmake -B build -DWAMR_BUILD_REF_TYPES=1 -DWAMR_BUILD_FAST_JIT=1
cmake --build build --parallel
sudo cmake --build build --target install
```
Clone and build the HarfBuzz project:
```bash
git clone https://github.com/harfbuzz/harfbuzz.git
```
For building the project get the tools according to this [guide](https://harfbuzz.github.io/building.html).

Then run the following commands.

```bash
meson setup build -Dwasm=enabled
meson compile -C build
```

Update the $HOME/Projects directory according to where you cloned your projects and add `libharfbuzz.so.0` and `libiwasm.so` to the `LD_PRELOAD`, :

```bash
echo 'export LD_PRELOAD="$HOME/Projects/harfbuzz-9.0.0/build/src/libharfbuzz.so.0 $HOME/Projects/wasm-micro-runtime-WAMR-2.1.1/product-mini/platforms/linux/build/libiwasm.so"' >> ~/.bashrc
```

Build the `wasmpoc.ttf`:

```bash
make -C wasmpocttf/
```

Add the font by copying the wasmpoc.ttf file into ~/.fonts and try it out using an editor of your choice.