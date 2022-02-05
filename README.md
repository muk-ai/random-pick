# randompick

Selects a file at random and returns it.  
In order to open that file, you need to provide instructions.

## macOS

```
#!/bin/sh

picked_file=$(./randompick-macos-amd64)
echo "$picked_file"
open "$picked_file"
```

## Windows


```
@echo off
chcp 65001
setlocal
for /f "usebackq delims=" %%A in (`randompick-windows-amd64.exe`) do set picked_file=%%A
echo "%picked_file%"
start "titlebar text" "%picked_file%"
```
