from argparse import ArgumentParser

from main import Node


def parse_args():
    parser = ArgumentParser()
    parser.add_argument('path')
    return parser.parse_args()


if __name__ == '__main__':
    args = parse_args()
    node = Node(args.path)

    path = [node]
    while True:
        while path[-1].dirs is None:
            path.pop()
        print('\033c', end='')
        print(f'Total {path[-1].size/2**20:.2f} Mb')
        for i, dir in enumerate(path[-1].dirs):
            print(f' {i+1:2} {dir.size/path[-1].size*100:2.0f}% {dir.name}')

        try:
            action = int(input('> '))
            if action:
                path.append(path[-1].dirs[action - 1])
            else:
                path.pop()
                if not path:
                    break
        except (ValueError, IndexError):
            continue
