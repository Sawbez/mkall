# mkall
Create multiple paths in a single command!

Usage:
```
mkall [PATHS]... [--help] [--version]
```

You can have as many paths as you want and they can be nested.
For example,
```
mkall path1/other/next this/is/nested path1/again/
```
will work successfully while a similar `mkdir` command
would not work successfully.
