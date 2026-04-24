import subprocess


def print_and_run(command: list[str]) -> None:
    print("Run command:", " ".join(command), " ...", end="", flush=True)
    process = subprocess.run(command, capture_output=True, text=True, timeout=200)
    if process.returncode == 0:
        print(" Ok.", flush=True)
    else:
        print(" Failed!", flush=True)
        print("-" * 5, "stdout", "-" * 5)
        print(process.stdout)
        print("-" * 5, "stderr", "-" * 5)
        print(process.stderr)
        raise RuntimeError(f"Return Code is not zero! Command:{' '.join(command)}")
