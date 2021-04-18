# rust-testing

Install using exe downloaded from 
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

Should probably have used this instead:
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe


Ran through example.

Opened in clion - worked fine. Issue debugging.

https://blog.jetbrains.com/clion/2019/10/debugging-rust-code-in-clion/

$ rustup toolchain list
stable-x86_64-pc-windows-msvc (default)

$ rustup default  stable-x86_64-pc-windows-gnu
...
info: default toolchain set to 'stable-x86_64-pc-windows-gnu'

  stable-x86_64-pc-windows-gnu installed - rustc 1.51.0 (2fd73fabe 2021-03-23)


$ rustup toolchain list
stable-x86_64-pc-windows-gnu (default)
stable-x86_64-pc-windows-msvc
