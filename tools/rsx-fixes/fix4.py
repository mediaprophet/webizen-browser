import os, re
d = 'C:/Projects/webizen-browser/webizen-studio/src/components'

def remove_inline_ifs(filename):
    p = os.path.join(d, filename)
    with open(p, 'r', encoding='utf-8') as f:
        c = f.read()
    # Replace anything looking like {if ... { "color1" } else { "color2" }} with simply "color1"
    # To be safe and simple, we'll just replace `{if ` to `{ if ` so Dioxus might parse it?
    # No, Dioxus doesn't support `{if` inside strings at all. 
    # We will use regex to find `{if ...}` inside style attributes.
    # Actually, we can just replace the whole style attribute since it's just mock styles!
    pass
