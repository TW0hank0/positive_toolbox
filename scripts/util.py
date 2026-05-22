import subprocess
import sys
import threading
import time

import colorama
from colorama import Fore, Style

colorama.init(autoreset=True)


def print_and_run(command: list[str]) -> None:
    prompt = f"{Style.BRIGHT}{Fore.GREEN}Run command: {Fore.RESET}{Style.NORMAL}{' '.join(command)} ... "
    print(prompt, end="\r", file=sys.stdout, flush=True)
    start_time = time.time()
    process = threading.Thread(target=_run, args=(command,))
    process.run()
    while True:
        if process.is_alive():
            sys.stdout.write(
                prompt
                + f"{Style.DIM}{str(int(time.time() - start_time))}secs{Style.NORMAL}"
                + "\r"
            )
            sys.stdout.flush()
            time.sleep(0.4)
        else:
            sys.stdout.write(
                prompt
                + "Ok."
                + f" {Style.DIM}({str(int(time.time() - start_time))}secs){Style.NORMAL}"
                + "\n"
            )
            sys.stdout.flush()
            break


def _run(cmd: list[str]):
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=3000)
    if p.returncode == 0:
        print(" Ok.", flush=True)
    else:
        print(" Failed!", flush=True)
        print("-" * 5, "stdout", "-" * 5)
        print(p.stdout)
        print("-" * 5, "stderr", "-" * 5)
        print(p.stderr)
        raise RuntimeError(f"Return Code is not zero! Command:{' '.join(cmd)}")
