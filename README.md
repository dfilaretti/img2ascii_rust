# img2ascii

Convert images into ASCII art.

User can provide additional options to specify the desired width in characters,
as well as the amount and type of horizontal correction to apply. (this is needed
because, unlike pixels which can be considered perfect squares, characters are more
tall than they are wide, meaning that directly mapping each pixel to a character will
result in a horizontally squeezed image).

## Build 

```
cargo build
```

## Basic usage 

Let's use our Mario as an example: 

<img src="imgs/mario.jpg" width="400">

Just type

```
cargo run -- -i imgs/mario.jpg 
```

Which gives us:

```
@@@o@@@o@@@@@@@@@@o@@@@@@@@@@@@@@@oo@@@o@@@@@@o@ooo@@@@@o@
@@@o@@oooo@oooo@@.o@@@@@@@@@@@@@@@oo@@@o@@@@@@ooooo@@@ooo@
@@o.o@oooooooo@@o.@@@@@@@@@@@@@@@@ooo. o@@@@@oooo@@@@oo@o@
@@o.o@ooooooo@@o.o@@@@@@@@@@@@@@@oo.    o@o@oooo@o@@oo@@oo
@@ooo@ooo@@o@@oo.o@@@@@@@@@@@@@@@@.  .. .o@@@@ooo@@@oo@@oo
@@@@oooo@@@@@oo..@@@@@@@@o@@@o@@@.  .... o@@@ooooo@oo@@ooo
@@@@@@o..oooooo.o@@@@o@@@o@@@@@o.   .... o@@ooooo@o.@@oooo
@@@@@oo     ..oo@@@@@@@@oooooo.      ..  o@oooooooooooo@@o
@@@@@@@o ..    .o@@ooo..       .         o@ooo.ooooooo@ooo
@o@@oooo.....           .    ...          oooo.oooooo@oooo
@@@@oo... ....   ...    oo.               ooo.ooo.oooooooo
@@@oo.           .     .o@o               .oo.o....ooooooo
oooo                   .@@@.       .       ..... ..ooooooo
 .o.                   .@@@o    . ..        ..    .ooooooo
  .         .   ..  .. o@@@@o.                    .ooooooo
.          ..    .... .@@@@@@oooooo.              ..oooooo
          ...        .@@@@@@oooo@@oo..            .ooooooo
          ...      ..@@ooooo...oooooo.            ..oooooo
         ....      .@@@o.... .oooooooo.....         .ooooo
         ..o.     .o@@@@o.. ......oooo.o...  ..     ...ooo
          .@o    ..oooooo..  .............           ..ooo
          o@@@..... ......................            ..oo
         .@@@@@@o.........................            ..oo
         .@@@@@@@ooooo....................            ..oo
         .@@@@@@@@ooooo....................           ...o
        .@@@@@@@@oooooooooo................           ...o
       .o@@@@@@@@oooooooooooo..............           ...o
      .oo@@@@@@@oooooooooooooo.o...........           ....
       .o@@@@@@@@oooooooo.ooooooooo.......            ....
        .o@@@@@@@oooooooooooooooooooo....            .....
         .o@@@@@@@ooooooooooooooooooo..             .....o
          .o@@@@@@@o...oooooooooooo..               ......
         .o@@@@@@@@oo..............                 ......
         o@@@@@@@@@@oo.    .......                    ....
         o@oo@@@@o@oooo    ......             ............
          .o@@@@@@@oooo.  ...oo..  ............oo.........
           .@@@@o@@oooo. ....oo....ooooooooooooooo........
            .oooo@o.... ..........oooooooooooooooo.oo....o
                ..       ....... ..ooooooooooooooo..oo..oo
                          .....o....ooooooooo@ooooo.ooo.oo
                            ..... ....oooooooooooooooooooo
                             .... .....oooooooo@ooooo..o@o
```

## More options 

The example above uses the default settings. To see all available options run 

```
cargo run -- -h
```

which will show all available configuration options: 

```
Convert an image file to ASCII art

Usage: img2ascii [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    Image file to convert
  -w, --width <WIDTH>    Desired width of the ASCII art (in characters) [default: 80]
  -m, --mode <MODE>      Horizontal adjustment mode [default: stretch] [possible values: stretch, repeat]
  -a, --amount <AMOUNT>  Horizontal adjustment amount [default: 2]
  -v, --verbose          Display debug information
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```
