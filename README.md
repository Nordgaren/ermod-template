# ermod template
Template for Elden Ring dll mod.

Set `MOD_NAME="modname"` 

Add dll to config and enable extension.
```toml
external_dlls = [ "mod/modname.dll" ]

[extension.modname]
enabled = true
```

More detailed instructions here:
https://github.com/Nordgaren/modengine2-ext-rs/blob/master/README.md

