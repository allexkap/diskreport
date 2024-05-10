import math
import tkinter as tk


def hidh_dpi_fix():
    import os

    if os.name == 'nt':
        from ctypes import windll

        windll.shcore.SetProcessDpiAwareness(1)


def eqgt(a, b):
    return 0 if a < b else 1 if a == b else 2


class PieDiagram(tk.Frame):
    colors = {
        'palette': (
            '#4572c4 #6d45c4 #c145c4 #c44572 #c46d45 '
            '#c4c145 #72c445 #45c46d #45c4c1 #c0c0c0 '
        ).split(),
        'border': '#ffffff',
        'background': '#ffffff',
    }

    def __init__(self, *args, values, **kwargs):
        self.sizes = (1000, 800)
        tk.Frame.__init__(self, *args, **kwargs)
        self.canvas = tk.Canvas(
            width=self.sizes[0],
            height=self.sizes[1],
            background=self.colors['background'],
        )
        self.canvas.pack()
        self.canvas.bind('<Button-1>', self.on_click)
        self.draw(values)

    def draw(self, values):
        total = sum(values)
        values = sorted(values, reverse=True)
        sectors = [value / total * 360 for value in values]

        radius = min(10**6, self.sizes[0] * 0.3)
        center = [self.sizes[0] / 2, self.sizes[1] / 2]
        center[1] += self.sizes[0] * 0.05
        pie_box = (
            center[0] - radius,
            center[1] - radius,
            center[0] + radius,
            center[1] + radius,
        )

        angle, dir = 90, -1
        for i, sector in enumerate(sectors):
            sector *= dir
            self.canvas.create_arc(
                *pie_box,
                start=angle,
                extent=sector,
                fill=self.colors['palette'][i % len(self.colors['palette'])],
                outline=self.colors['border'],
            )

            text_angle = angle + sector / 2
            anchor = ('e', 'center', 'w')[eqgt((text_angle - 90) % 360, 180)]
            self.canvas.create_text(
                center[0] + math.cos(math.radians(text_angle)) * (radius + 10),
                center[1] - math.sin(math.radians(text_angle)) * (radius + 10),
                anchor=anchor,
                text='Divinity Original Sin 10',
            )
            angle += sector

    def on_click(self, event):
        n = self.canvas.find_overlapping(event.x, event.y, event.x, event.y)
        if n:
            print(n)


if __name__ == '__main__':
    hidh_dpi_fix()
    root = tk.Tk()
    root.resizable(False, False)
    root.bind('<Escape>', lambda e: exit())
    pie = PieDiagram(root, values=[1] * 9).pack()
    root.mainloop()
