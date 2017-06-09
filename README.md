# nuklear-rust

[![Latest Version](https://img.shields.io/crates/v/nuklear-rust.svg)](https://crates.io/crates/nuklear-rust)

The bindings to the [Nuklear](https://github.com/vurtun/nuklear) 2D immediate GUI library. 

Currently beta.

Drawing backends:
* [gfx](https://github.com/snuk182/nuklear-backend-gfx) for [GFX 3D drawing engine](https://github.com/gfx-rs/gfx) (examples: [OpenGL](https://github.com/snuk182/nuklear-test), [DX11](https://github.com/snuk182/nuklear-test/tree/dx11))
* [glium](https://github.com/snuk182/nuklear-backend-glium) for [glium 3D drawing engine](https://github.com/tomaka/glium) (example: [here](https://github.com/snuk182/nuklear-test/tree/glium))
* [GDI](https://github.com/snuk182/nuklear-backend-gdi) for [WinAPI GDI](https://msdn.microsoft.com/en-us/library/windows/desktop/dd145203(v=vs.85).aspx) (example: [here](https://github.com/snuk182/nuklear-test/tree/gdi))

All examples are based on the [extended example](https://github.com/vurtun/nuklear/blob/master/example/extended.c) of the original library.

API suggestions, PRs and improvement ideas are welcome! Don't have much experience of building highly capable UI libraries yet.
