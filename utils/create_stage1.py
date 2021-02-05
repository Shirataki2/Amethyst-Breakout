WIDTH = 0.08
HEIGHT = 0.03

INITIAL_X = 0.10
INITIAL_Y = 0.95

END_X = 0.90
END_Y = 0.56

print("""Stage(\n\tblocks: [""")

y = INITIAL_Y
x = INITIAL_X
s = 0

while y > END_Y:
    x = INITIAL_X
    while x < END_X:
        print(f"""\t\t(\n\t\t\tx: {round(x, 2)},\n\t\t\ty: {round(y, 2)},\n\t\t\tsprite: {s}\n\t\t),""")
        x += WIDTH
    y -= HEIGHT
    s += 11
    s %= 110

print("""\t]\n)""")
