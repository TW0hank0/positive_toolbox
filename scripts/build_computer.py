# SPDX-License-Identifier: AGPL-3.0-only
# 著作權所有 (C) 2026 TW0hank0
#
# 本檔案屬於 positive_mahjong 專案的一部分。
# 專案儲存庫：https://github.com/TW0hank0/positive_mahjong
#
# 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
# 第 3 版（僅此版本）重新發佈及/或修改本程式。
#
# 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
# 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
#
# 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
# 如果沒有，請參見 <https://www.gnu.org/licenses/>。

import subprocess
import sys
import time
import os

import zip_files


def main():
    start_time = time.time()
    #
    commands = [
        ["cargo", "build", "--workspace", "--release"],
        [
            "uv",
            "run",
            "pyinstaller",
            os.path.join("ptb_launcher", "ptb_launcher.spec"),
        ],
    ]
    for command in commands:
        print(f"Run Command:{' '.join(command)} ...", end="")
        sys.stdout.flush()
        process = subprocess.run(
            command,
            stdout=sys.stdout,
            stdin=sys.stdin,
            stderr=sys.stderr,
        )
        if process.returncode != 0:
            print("Error!")
            print("--- stdout ---")
            print(process.stdout.decode())
            print("--- stderr ---")
            print(process.stderr.decode())
            sys.exit(1)
        print("Ok!")
    #
    print("zip-files ...", end="")
    sys.stdout.flush()
    zip_files.main()
    print("Ok!")
    print("-" * 10)
    print("finish in", time.time() - start_time)


if __name__ == "__main__":
    main()
