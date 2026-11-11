"""Draw the Arrival Card Bot avatar: "Stampy", a smiling passport with an orbiting paper plane.

Usage: python tools/make_avatar.py [out_dir]   (needs Pillow)
Writes avatar.png (1024 px still), avatar.gif and avatar.webp (512 px, 2.2 s loop).
"""

import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

BG, DISC, SHADOW = "#1f2a63", "#2a3785", "#151d4a"
COVER, SPINE, GOLD = "#17b890", "#0f9c7a", "#ffd166"
INK, CHEEK, STAMP = "#12304a", "#ff8fa3", "#ff5a5f"
FRAMES, FRAME_MS = 36, 60
STARS = [(0.13, 0.2, 0.0), (0.86, 0.15, 0.3), (0.9, 0.8, 0.6), (0.1, 0.83, 0.8), (0.5, 0.045, 0.45)]


def ease_pop(x: float) -> float:
    """0 -> 1 with an overshoot (a rubber stamp landing)."""
    x = min(max(x, 0.0), 1.0)
    return 1 + 2.4 * (x - 1) ** 3 + 1.4 * (x - 1) ** 2


def render(t: float, size: int, still: bool = False, ss: int = 3) -> Image.Image:
    n = size * ss
    img = Image.new("RGBA", (n, n), BG)
    d = ImageDraw.Draw(img, "RGBA")
    u = lambda v: v * n  # unit square -> pixels

    def circle(x, y, r, **kw):
        d.ellipse([u(x - r), u(y - r), u(x + r), u(y + r)], **kw)

    circle(0.5, 0.5, 0.47, fill=DISC)
    for x, y, phase in STARS:
        tw = 0.5 + 0.5 * math.sin(2 * math.pi * (t + phase))
        circle(x, y, 0.006 + 0.006 * tw, fill=(255, 255, 255, int(110 + 120 * tw)))

    # dashed flight path
    orbit = 0.385
    for k in range(40):
        a = 360 / 40 * k
        d.arc([u(0.5 - orbit), u(0.5 - orbit), u(0.5 + orbit), u(0.5 + orbit)], a, a + 4.5,
              fill=(255, 255, 255, 80), width=int(u(0.007)))

    # passport body, bobbing
    dy = 0.012 * math.sin(2 * math.pi * t)
    cx, cy, w, h = 0.5, 0.5 + dy, 0.34, 0.44
    box = lambda ox=0.0, oy=0.0: [u(cx - w / 2 + ox), u(cy - h / 2 + oy), u(cx + w / 2 + ox), u(cy + h / 2 + oy)]
    d.rounded_rectangle(box(0.014, 0.018), radius=u(0.05), fill=SHADOW)
    d.rounded_rectangle(box(), radius=u(0.05), fill=COVER)
    spine = Image.new("RGBA", (n, n), (0, 0, 0, 0))
    ImageDraw.Draw(spine).rounded_rectangle(box(), radius=u(0.05), fill=SPINE)
    spine = spine.crop((0, 0, int(u(cx - w / 2 + 0.05)), n))
    img.alpha_composite(spine)

    fx = cx + 0.022  # face centre, shifted off the spine
    # gold globe emblem
    gy, gr, lw = cy - 0.135, 0.036, int(u(0.008))
    circle(fx, gy, gr, outline=GOLD, width=lw)
    d.ellipse([u(fx - gr * 0.45), u(gy - gr), u(fx + gr * 0.45), u(gy + gr)], outline=GOLD, width=lw)
    d.line([u(fx - gr), u(gy), u(fx + gr), u(gy)], fill=GOLD, width=lw)

    # face
    blink = (not still) and 0.60 <= t < 0.66
    for sx in (-0.062, 0.062):
        ex, ey = fx + sx, cy - 0.01
        if blink:
            d.line([u(ex - 0.022), u(ey), u(ex + 0.022), u(ey)], fill=INK, width=int(u(0.011)))
        else:
            circle(ex, ey, 0.023, fill=INK)
            circle(ex + 0.008, ey - 0.008, 0.008, fill="white")
        circle(fx + sx * 1.75, cy + 0.045, 0.021, fill=CHEEK)
    d.arc([u(fx - 0.045), u(cy - 0.01), u(fx + 0.045), u(cy + 0.07)], 25, 155, fill=INK, width=int(u(0.012)))

    # "PASSPORT" lettering, as two gold bars
    for by, bw in ((0.135, 0.17), (0.17, 0.10)):
        d.rounded_rectangle([u(fx - bw / 2), u(cy + by - 0.009), u(fx + bw / 2), u(cy + by + 0.009)],
                            radius=u(0.009), fill=GOLD)

    # the arrival stamp: thunks down, holds, fades
    if still:
        scale, alpha = 1.0, 1.0
    elif 0.30 <= t < 0.97:
        scale = ease_pop((t - 0.30) / 0.12)
        alpha = 1.0 if t < 0.85 else 1 - (t - 0.85) / 0.12
    else:
        scale, alpha = 0.0, 0.0
    if scale > 0:
        sx, sy, r = 0.665, 0.665 + dy, 0.088 * scale
        layer = Image.new("RGBA", (n, n), (0, 0, 0, 0))
        ld = ImageDraw.Draw(layer)
        ld.ellipse([u(sx - r), u(sy - r), u(sx + r), u(sy + r)], fill="#fff4ef", outline=STAMP, width=int(u(0.016 * scale)))
        ld.ellipse([u(sx - r * 0.74), u(sy - r * 0.74), u(sx + r * 0.74), u(sy + r * 0.74)], outline=STAMP,
                   width=int(u(0.006 * scale)))
        check = [(sx - r * 0.38, sy + r * 0.02), (sx - r * 0.1, sy + r * 0.3), (sx + r * 0.42, sy - r * 0.3)]
        ld.line([(u(x), u(y)) for x, y in check], fill=STAMP, width=int(u(0.022 * scale)), joint="curve")
        for x, y in check:
            ld.ellipse([u(x - 0.011 * scale), u(y - 0.011 * scale), u(x + 0.011 * scale), u(y + 0.011 * scale)], fill=STAMP)
        layer = layer.rotate(14, center=(u(sx), u(sy)), resample=Image.BICUBIC)
        if alpha < 1:
            layer.putalpha(layer.getchannel("A").point(lambda a: int(a * alpha)))
        img.alpha_composite(layer)
        d = ImageDraw.Draw(img, "RGBA")

    # paper plane on the orbit, with a dotted trail
    th = 2 * math.pi * (0.1 if still else t) - math.pi / 2
    for k in range(1, 7):
        a = th - 0.085 * k - 0.1
        circle(0.5 + orbit * math.cos(a), 0.5 + orbit * math.sin(a), 0.011 * (1 - k / 8),
               fill=(255, 255, 255, int(230 * (1 - k / 7))))
    px, py, heading = 0.5 + orbit * math.cos(th), 0.5 + orbit * math.sin(th), th + math.pi / 2

    def at(lx, ly):
        c, s = math.cos(heading), math.sin(heading)
        return (u(px + lx * c - ly * s), u(py + lx * s + ly * c))

    nose, notch = (0.075, 0.0), (-0.03, 0.0)
    d.polygon([at(*nose), at(-0.06, -0.058), at(*notch)], fill="white")
    d.polygon([at(*nose), at(-0.06, 0.058), at(*notch)], fill="#c9d4ff")
    d.polygon([at(*nose), at(-0.06, 0.016), at(*notch)], fill="#9fb0f5")

    return img.resize((size, size), Image.LANCZOS).convert("RGB")


def main() -> None:
    out = Path(sys.argv[1] if len(sys.argv) > 1 else "assets")
    out.mkdir(parents=True, exist_ok=True)
    render(0.0, 1024, still=True).save(out / "avatar.png")
    frames = [render(i / FRAMES, 512) for i in range(FRAMES)]
    frames[0].save(out / "avatar.webp", save_all=True, append_images=frames[1:], duration=FRAME_MS, loop=0, quality=90)
    pal = [f.quantize(colors=128, dither=Image.Dither.NONE) for f in frames]
    pal[0].save(out / "avatar.gif", save_all=True, append_images=pal[1:], duration=FRAME_MS, loop=0, optimize=True)
    for name in ("avatar.png", "avatar.gif", "avatar.webp"):
        print(f"wrote {out / name} ({(out / name).stat().st_size // 1024} KB)")


if __name__ == "__main__":
    main()
