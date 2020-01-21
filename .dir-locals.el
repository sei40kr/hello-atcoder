;;; .dir-locals.el ---                               -*- lexical-binding: t; -*-

;; Copyright (C) 2018 Seong Yong-ju

;; Author: Seong Yong-ju <sei40kr@gmail.com>

((rustic-mode
  . ((rustic-racer-rust-src-path . "~/.rustup/toolchains/1.15.1-x86_64-apple-darwin/lib/rustlib/src/rust/src")
     (quickrun-option-cmd-alist
      . ((:command . "rustup")
         (:exec    . ("%c run --install 1.15.1 rustc %o -O -o %e %s"
                      "RUST_BACKTRACE=1 atcoder-tools test -e %e -d %d"))
         (:remove . ("%e"))
         (:tempfile . nil)
         (:description . "Compile Rust file with Rustup/rustc and test with atcoder-tools"))))))
