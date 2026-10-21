# CDR editing foundation

Initial development snapshot, 2026-10-21. The application still carries its upstream name, VectorCraft. This is a development branch, not a finished replacement for another product.

## Implemented in this branch

- CDR is registered as an experimental read-only format in Open and Place.
- A pinned TraceDraw parser reads CDR and converts the first page through SVG into editable artwork.
- Warnings identify conversion approximations, skipped objects and omitted pages.
- Native saves use the application's working format; this branch does not write CDR.
- The upstream dedicated Windows 7 SP1 x64 packaging path is retained.

## Compatibility requirements

| Target | Build path | Runtime verification in this project |
| --- | --- | --- |
| Windows 7 SP1 x64 | packaging/windows/windows7.ps1, dedicated Rust win7 target, static CRT, OpenGL 3.3 | Not tested |
| Windows 10 x64 | Upstream modern Windows build | Not tested |
| Windows 11 x64 | Upstream modern Windows build | Not tested |
| Mobile | Upstream browser foundation | Touch workflows not adapted or tested |

Windows 7 support is a requirement, not a verified claim. Native Open/Save, clipboard, fonts, printing and large files must pass on each Windows version. A successful compile cannot establish runtime compatibility. No Windows 7 x86 build is included.

## Next development gates

1. Parser bridge and engine integration compile; five focused tests passed on Linux. Real-file and Windows checks remain.
2. Build the normal desktop app and the separate Windows 7 package.
3. Open real X7 files and compare dimensions, editable text, curves, fills, clipping and bitmap placement against user-supplied exports.
4. Implement all-page import with page selection and preserve more of the document model directly instead of the SVG bridge.
5. Adapt the workspace to label production: property bar, millimetre/inch sizing, object layout and step-and-repeat.
6. Add verified CDR output and automation compatibility only after round-trip evidence. Existing VBA/GMS macros are not supported by this branch.
7. Adapt and test the mobile interface after the shared document model is stable.

## ArtCraft study

The source uses a layered Rust workspace: geometry and colour, document model, tools and effects, renderers and file formats, command engine with undo, and separate UI/CLI/MCP/browser entry points. This branch follows that structure rather than presenting a browser mockup as a desktop application.

The upstream project describes itself as alpha and publishes gaps and self-assessed parity estimates. Its clean-room contribution rules use public documentation and do not use Adobe binaries. The inspected sources do not establish that the suite was produced by feeding binaries into REA.

Sources reviewed:
- https://github.com/storytold/vectorcraft (base commit bc1ae0825264c5ca4b27bd718d6e04f2121b8bb3)
- https://github.com/storytold/vectorcraft/blob/main/AGENTS.md
- https://github.com/storytold/vectorcraft/blob/main/docs/architecture.md
- https://github.com/storytold/vectorcraft/blob/main/docs/windows7.md
- https://github.com/storytold/photocraft
- https://github.com/igtoth/tracedraw (parser commit 255cd805cc72a75f5d5b6d429ebf54e54c60fb21)
- https://github.com/morluto/rea
- https://doc.rust-lang.org/rustc/platform-support/win7-windows-msvc.html

No proprietary application binaries, resources or decompiled output are included in the source package. The user's uploaded program files remain analysis inputs outside this project.
