# rust-testing

## Install
Install using exe downloaded from 
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

Should probably have used this instead:
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe


Ran through example.

Opened in clion - worked fine. Issue debugging.

https://blog.jetbrains.com/clion/2019/10/debugging-rust-code-in-clion/

```shell
$ rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
```

```shell
$ rustup default  stable-x86_64-pc-windows-gnu
...
info: default toolchain set to 'stable-x86_64-pc-windows-gnu'

  stable-x86_64-pc-windows-gnu installed - rustc 1.51.0 (2fd73fabe 2021-03-23)
```
```shell
$ rustup toolchain list
stable-x86_64-pc-windows-gnu (default)
stable-x86_64-pc-windows-msvc
```

## Updating

```shell
 rustup --version
rustup 1.23.1 (3df2264a9 2020-11-30)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.51.0 (2fd73fabe 2021-03-23)`
```

```shell
$ rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-gnu'
info: latest update on 2023-12-28, rust version 1.75.0 (82e1608df 2023-12-21)
...
   stable-x86_64-pc-windows-gnu updated - rustc 1.75.0 (82e1608df 2023-12-21) (from rustc 1.51.0 (2fd73fabe 2021-03-23))
  stable-x86_64-pc-windows-msvc updated - rustc 1.75.0 (82e1608df 2023-12-21) (from rustc 1.51.0 (2fd73fabe 2021-03-23))

info: cleaning up downloads & tmp directories
```

```shell
$ rustup --version
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.75.0 (82e1608df 2023-12-21)`
```

Update clion to 2023-3-2 and installed rust plugin.
