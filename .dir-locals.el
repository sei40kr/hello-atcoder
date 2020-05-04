;;; .dir-locals.el ---                               -*- lexical-binding: t; -*-

;; Copyright (C) 2018 Seong Yong-ju

;; Author: Seong Yong-ju <sei40kr@gmail.com>

((rustic-mode . ((rustic-lsp-server . rls)
                 (lsp-disabled-clients . (rust-analyzer))
                 (lsp-rust-server . rls)
                 (lsp-rust-sysroot .  "~/.rustup/toolchains/1.42.0-x86_64-apple-darwin")
                 (quickrun-option-cmd-alist . ((:command . "rustup")
                                               (:exec    . ("%c run --install 1.42.0 rustc %o -O -o %e %s"
                                                            "RUST_BACKTRACE=1 atcoder-tools test -e %e -d %d"))
                                               (:remove . ("%e"))
                                               (:tempfile . nil)
                                               (:description . "Compile Rust file with Rustup/rustc and test with atcoder-tools"))))))
