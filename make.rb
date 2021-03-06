#!/usr/bin/env ruby
require "ruby_make_script"

make do
    :update_raw .then do
        rm "-rf", "target/debug/build/lkm-*"
        cd 'linux-kernel-module-rust'
        r "cargo build -p lkm --features bindgen"
        cd ".."
        cp *Dir.glob("target/debug/build/lkm-*/out/bindings.rs"), "linux-kernel-module-rust/src/bindings_raw.rs"
    end
end