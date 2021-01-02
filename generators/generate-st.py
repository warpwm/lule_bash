#!/usr/bin/env python3
import json
from os.path import expanduser
with open(expanduser('~/.cache/wal/colors.json')) as raw_json:
    lule_json = json.load(raw_json)
first16 = []
for i in range(0, 17):
    first16.append(lule_json['colors'][f'color{i}'])
header = ""
header += "const char *colorname[] = {"
for index, color in enumerate(first16):
    header += f"[{index}] = \"{color}\", \n"
header += """
[256] = "#000000",
[257] = "#f4f4f4",
[258] = "#f4f4f4"
};
unsigned int defaultbg = 0;
unsigned int defaultfg = 257;
unsigned int defaultcs = 258;
unsigned int defaultrcs= 258;
"""
with open(expanduser('~/.cache/wal/colors-wal-st.h'), 'w') as colors:
    colors.write(header)

