;;; .dir-locals.el ---                               -*- lexical-binding: t; -*-

;; Copyright (C) 2018 Seong Yong-ju

;; Author: Seong Yong-ju <sei40kr@gmail.com>

((haskell-mode . ((eval . (setq quickrun-option-cmd-alist
                            (list
                              '(:command . "stack")
                              (cons :exec
                                (list
                                  "%c ghc -- %o -o %e %s"
                                  (concat (projectile-project-root) "utils/run-tests.sh %e %a")))
                              '(:compile-only . "%c ghc -- %o -o %e %s")
                              '(:remove "%e")
                              '(:description . "Compile Haskell file and execute via stack ghc"))))))
  (rust-mode . ((racer-rust-src-path . "~/.rustup/toolchains/1.15.1-x86_64-apple-darwin/lib/rustlib/src/rust/src")
                 (eval . (setq quickrun-option-cmd-alist
                           (list
                             '(:command . "rustup")
                             (cons :exec
                               (list
                                 "%c run --install 1.15.1 rustc %o -o %e %s"
                                 (concat (projectile-project-root) "utils/run-tests.sh %e %a")))
                             '(:compile-only . "%c run --install 1.15.1 rustc %o -o %e %s")
                             '(:remove . ("%e"))
                             '(:description . "Compile rust and execute via rustup")))))))
