cd ..
mkdir sfx
7z a sfx\c.7z target\release\telegramr.exe -m0=lzma2 -mx
copy /b tools\7zS2.sfx + sfx\c.7z  sfx\tr.exe
tools\ResourceHacker.exe  -open sfx\tr.exe -save sfx\tr.exe -action addoverwrite -res icon.ico -mask ICONGROUP,1,
cd scripts