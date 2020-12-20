

<img align="right" width="26%" src="https://i.imgur.com/1QLEFnH.png">

@ lule
===

A bash(y) alternative to [pywal](https://github.com/dylanaraps/pywal) by mostly using [pastel](https://github.com/sharkdp/pastel) to generate the full 255 colors from an image. 

```
lule --image=/some/path/of/an/image.jpg --palette=convert set
```
<hr>

![](/resources/a_gif.gif)

<hr>

# Why another X?

The advantages from __pywal__ are that, __lule__ genretarates all 255 colors from an image and the other BIG BIG advantage is that it's composed of many tools and all are in a bash script, so you can chnage anything. All the tools are binaries so you're saved form dependency hell.

# USAGE

```
lule v.0.1
a command line to set 255 colors on tty's and other places that use ANSI colors

USAGE:
    lule [options][flags] <command>
    lule <special> [flags]

OPTIONS:
    --palette[=]<name> -> { pigmnts } , schemer2 , convert , imgscheme , 
        specify the palete binary to use
    --configs[=]<filepath> :: or specify $LULE_C as environment variable
        specify a config file where to load color preferences
    --scheme[=]<name> -> default , 
        specify the scheme form configs
    --sort[=]<name> -> brightness , luminance , hue , { chroma } , random , 
        specify the soring colord of pallete
    --saturate[=]<value> -> only numbers (0.0-1.0) are valid
        ammout of saturation of main colors
    --image[=]<filepath>
        specify the image to extract colors from
    --script[=]<filepath> :: or specify $LULE_S as environment variable
        specify an external script to run after colors are genrated
    --wallpath[=]<dirpath> :: or specify $LULE_W as environment variable
        specify a folder to pick an image randomly
    --loop[=]<seconds> -> only numbers are valid
        loop thrugh direcory (needs $LULE_W or $--wallpath)

FLAGS:
    -n  dont set wallpaper
    -d  dont set colors
    -p  read values from pipe/stdin
    -f  refresh the colors  * 
    -r  reset default colors  * 
  flags marked with:  *  should be used alone eg.:  lule -r 

COMMANDS:
    set          generate new colors from new image
    pick         pick a color as accent color
    regen        generate new colors from same image
    theme        invert dark and light theme

SPECIAL:
    colors [flags]       print all 255 colors in terminal  * 
    palette [flags]      more info about diffenert palette generators  * 
    configs [flags]      set and save color configure options * 
  commands marked with  *  have their own flags, check: lule <special> -h 
```
