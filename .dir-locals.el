;;; .dir-locals.el ---                               -*- lexical-binding: t; -*-

;; Copyright (C) 2018 Seong Yong-ju

;; Author: Seong Yong-ju <sei40kr@gmail.com>

((haskell-mode . ((quickrun-option-cmd-alist . ((:command . "stack")
                                                 (:exec "%c runghc %o %s %a")))))
  (rust-mode . ((eval . (flycheck-disable-checker 'rust))
                 (quickrun-option-cmd-alist . ((:command . "rustup")
                                                (:exec . ("%c command --install 1.15.1 rustc %o -o %e %s" "%e %a"))
                                                (:compile-only . "%c command --install 1.15.1 rustc %o -o %e %s")
                                                (:remove . ("%e"))
                                                (:description . "Compile rust and execute via rustup")))
                 (racer-rust-src-path . "~/.rustup/toolchains/1.15.1-x86_64-apple-darwin/lib/rustlib/src/rust/src"))))
