#!/usr/bin/env python3
"""Generate extra zeish wallpapers. Run locally: python3 generate.py"""

import math
import os

from PIL import Image

SIZE = (2560, 1440)
OUT_DIR = os.path.dirname(os.path.abspath(__file__))


def lerp(a, b, t):
    return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))


def diagonal_gradient(name, top_left, bottom_right):
    w, h = SIZE
    img = Image.new("RGB", SIZE)
    px = img.load()
    denom = w + h
    for y in range(h):
        for x in range(0, w, 2):
            t = (x + y) / denom
            color = lerp(top_left, bottom_right, t)
            px[x, y] = color
            if x + 1 < w:
                px[x + 1, y] = color
    img.save(os.path.join(OUT_DIR, name))


def radial_glow(name, bg, glow):
    w, h = SIZE
    cx, cy = w / 2, h / 2
    max_r = math.hypot(cx, cy)
    img = Image.new("RGB", SIZE)
    px = img.load()
    for y in range(h):
        for x in range(0, w, 2):
            r = math.hypot(x - cx, y - cy) / max_r
            t = min(1.0, r)
            color = lerp(glow, bg, t)
            px[x, y] = color
            if x + 1 < w:
                px[x + 1, y] = color
    img.save(os.path.join(OUT_DIR, name))


def horizontal_bands(name, colors):
    w, h = SIZE
    img = Image.new("RGB", SIZE)
    px = img.load()
    n = len(colors)
    band_h = h / n
    for y in range(h):
        i = min(n - 1, int(y / band_h))
        j = min(n - 1, i + 1)
        local_t = (y - i * band_h) / band_h
        color = lerp(colors[i], colors[j], local_t)
        for x in range(0, w, 2):
            px[x, y] = color
            if x + 1 < w:
                px[x + 1, y] = color
    img.save(os.path.join(OUT_DIR, name))


if __name__ == "__main__":
    diagonal_gradient("zeish-wallpaper-slate.png", (22, 27, 34), (56, 68, 84))
    radial_glow("zeish-wallpaper-glow.png", (13, 17, 23), (35, 90, 120))
    horizontal_bands(
        "zeish-wallpaper-dusk.png",
        [(28, 20, 43), (73, 35, 89), (196, 90, 90), (240, 170, 90)],
    )
    print("wrote 3 wallpapers to", OUT_DIR)
