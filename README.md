Pixel art editor
================

Unsatisfied wih most free linux compatible pixel art editors, I started one
my own. This will be a pretty simple editor, most important features will be:
 * Loading palettes from files and exporting palettes
 * Palette generation
 * Multiple preview windows
 * Simple support for frames
 * Keyboard driven (Basically, the only thing which should require the mouse
should be to draw)

Unfortunately, the current libpng binding for rust haven't implemented a
decoder yet, so I might have to write my own.
