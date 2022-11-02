import sys


def run(args):
    print(f"I am {args[0]} called with arguments: {args[1:]}")


if __name__ == "__main__":
    run(sys.argv)
