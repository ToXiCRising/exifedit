# Exifedit 

<p align="center">
  <img src="recources\ExifToolIcon.png">
</p>

A very small gui that calls exiftool to edit metadata of analog pictures. Build with Rust and Slint. Currently only tested in Windows 10 and requires exiftool to be placed in PATH.

<p align="center">
  <img src="recources\ui_preview.jpg", width=500>
</p>

# Future Features and Ideas
- [ ] Color coding of carousel titles based on selection, and if writing of data was succesful
- [ ] a way to remove pictures from current session

- [x] dynamic exif tile system

- [ ] loading bar? 
- [x] implement image compression or better caching to optimize responsivenes

# Bugfixes
- [x] Crashes if you exit out of the file selector dialog

## Stuff
- Lazy static explanation for datahandler: 
   - https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

- Adding the Icon:
   - https://anthropicstudios.com/2021/01/05/setting-a-rust-windows-exe-icon/
   - https://stackoverflow.com/questions/50642574/how-can-i-specify-linker-flags-arguments-in-a-build-script

- Colors (Fluent UI Palette):
   - https://fluentuipr.z22.web.core.windows.net/heads/master/theming-designer/index.html