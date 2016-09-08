Pixel art editor
================

    23:24:07 lolirofle | Vrf Pixel art editor ens. Behöver?
    23:24:13     pranz | Ja
    23:24:24 lolirofle | Jag med

Unsatisfied wih most free linux compatible pixel art editors, I started one
my own. This will be a pretty simple editor, most important features will be:
 * Loading palettes from files and exporting palettes
 * Palette generation
 * Multiple preview windows
 * Simple support for frames
 * Keyboard driven (Basically, the only thing which should require the mouse
should be to draw)

~~Unfortunately, the current libpng binding for rust haven't implemented an
encoder yet, so I might have to write my own.~~

Piston had a work in progress encoder that @Lolirofle forked and did something
with. Yay!

## Contributors
 * [Lolirofle](https://github.com/Lolirofle)
