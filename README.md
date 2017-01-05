# Gemini

The Gemini project is an attempt at making a GEM-inspired,
but not a GEM cloned,
user interface toolkit.

The problems with most GUI toolkits are the *insane* dependency graphs, or the lack of a truly easy to use API.
For example, I was unsuccessful in getting gtk-rs installed because of dependency hell on my Ubuntu machine.
And conrod was a non-starter because, well, I just couldn't figure out where to start.
Another problem is that just about all of them I've seen (save gtk-rs) used floating point for coordinates.
That's great if you're rendering 3D stuff.
It's not what I need, though.

Rather than wait for these problems to be addressed,
I decided to write my own GUI toolkit.
Inspiration, in varying measures, comes from the following sources:

- Intuition (AmigaOS).
- GEOS (Commodore 64/128).
- GEM (Atari ST, IBM PC-compatibles).

I'm most impressed with how well the GEM environment is factored,
so I thought it made the most sense to more or less pick up from where it left off.
However, GEM has some issues which desperately need repair:

- GEM busy-waits to see if you're single-clicking or double-clicking.  This leads to laggy/unresponsive UIs.
- GEM refers to various resources, like fonts and such, by numeric index relative to a static configuration.  Altering this configuration requires actually restarting GEM itself.
- More I've probably since forgotten, I'm sure.

So this is my attempt to take the best parts of GEM,
and combine them with the best parts of GEOS and Intuition.

