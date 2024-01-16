# pikselsel
A [pixelflut](https://github.com/defnull/pixelflut) client written in Rust. It currently supports drawing images, gifs (kinda), rendering text, wiping the canvas, and tiling in both directions.

The name comes from the turkish words for "pixel" and "flood", which are "piksel" and "sel" respectively. This is basically a rust version of my other client, pyxelflut (written in python), and I aim to fully add the feature set of that client into this one.

TODO (sorted in order of priority):
- add delta optimization for gifs
- add command line arguments (pretty much done, just need to iron out the wrinkles
- make command generation faster
- fix that text bug
- add the ability to switch between all of the features (tiling, random placement etc)
- add threading
