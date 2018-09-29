# Modscript Interactive
REPL for modscript. Arguments are file paths for modscript packages to be compiled and attached.

Very simple at the moment. You can attach packages of functions to run, and then write statements.
Each statement must fit on a single line, you can run lists of statements by putting them in a scope {}.

If you write an expression (without a closing semicolon), it will return the value.

You can write `import x as y` to import package files, where `x` is the file name, and `y` is what you want to nickname the package. Alternatively, write `import x` to import the package with no nickname.

## TODO
* Curses support
* Multi-line statements (;;)
