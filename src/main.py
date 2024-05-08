import os
from pathlib import Path


class Node:
    def __init__(self, path: str, _entry: os.DirEntry | None = None) -> None:
        self.name = Path(path).absolute().name if _entry is None else _entry.name
        try:
            if _entry is None or _entry.is_dir():
                self.dirs = tuple(Node(entry.path, entry) for entry in os.scandir(path))
                self.size = sum(node.size for node in self.dirs)
            else:
                self.dirs = None
                self.size = _entry.stat().st_size
        except Exception as ex:
            self.dirs = None
            self.size = 0
            self.err = ex
        else:
            self.err = None

    def __str__(self) -> str:
        if self.err is None:
            return f'{self.name}: {self.size//2**20} Mb'
        return f'{self.name}: {self.err}'
