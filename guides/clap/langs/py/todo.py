import argparse

parser = argparse.ArgumentParser(description="A tiny to-do list")
parser.add_argument("-v", "--verbose", action="store_true", help="print more detail")
sub = parser.add_subparsers(dest="command", required=True)
add = sub.add_parser("add", help="add a task")
add.add_argument("text", help="what to do")
add.add_argument("-p", "--priority", type=int, default=2, help="1 (high) to 3 (low)")
sub.add_parser("list", help="list tasks")

args = parser.parse_args()
if args.command == "add":
    print(f'added "{args.text}" with priority {args.priority}')
    if args.verbose:
        print("(verbose mode is on)")
else:
    print("nothing to list yet")
