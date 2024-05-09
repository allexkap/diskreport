import tkinter as tk


def hidh_dpi_fix():
    import os

    if os.name == 'nt':
        from ctypes import windll

        windll.shcore.SetProcessDpiAwareness(1)


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
        self.length = 400
        self.offset = 20
        tk.Frame.__init__(self, *args, **kwargs)
        self.canvas = tk.Canvas(
            width=self.length,
            height=self.length,
            background=self.colors['background'],
        )
        self.canvas.pack()
        self.draw(values)

    def draw(self, values):
        total = sum(values)
        values = sorted(values, reverse=True)
        sectors = [value / total * 360 for value in values[:9]]
        if len(values) >= 9:
            sectors.append(sum(values[9:]) / total * 360)

        angle, dir = 90, -1
        for i, sector in enumerate(sectors):
            sector *= dir
            self.canvas.create_arc(
                self.offset,
                self.offset,
                self.length - self.offset,
                self.length - self.offset,
                start=angle,
                extent=sector,
                fill=self.colors['palette'][i],
                outline=self.colors['border'],
            )
            angle += sector


if __name__ == '__main__':
    hidh_dpi_fix()
    root = tk.Tk()
    root.resizable(False, False)
    pie = PieDiagram(root, values=(1, 3, 2, 1)).pack()
    root.mainloop()
