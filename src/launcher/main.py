import os
import subprocess
import sys


def main():
    main_exec_path = os.path.join(
        os.path.dirname(__file__), "positive_toolbox"
    )
    if os.name == "nt":
        main_exec_path = main_exec_path + ".exe"
    #
    subprocess.run(
        [main_exec_path],
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )


if __name__ == "__main__":
    main()
