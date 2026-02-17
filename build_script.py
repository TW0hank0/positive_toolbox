import subprocess
import sys


def main():
    subprocess.run(
        [
            "cargo",
            "generate",
            "--output-file",
            "ThirdPartyLicense.html",
            "about.hbs",
            "--threshold",
            "1.0",
        ],
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )
    subprocess.run(["uv", "run", "pyinstaller", "ptb_launcher.spec"])
    subprocess.run(["cargo", "build", "--release"])


if __name__ == "__main__":
    main()
