# SPDX-License-Identifier: AGPL-3.0-only
# 著作權所有 (C) 2026 TW0hank0
#
# 本檔案屬於 positive_toolbox 專案的一部分。
# 專案儲存庫：https://github.com/TW0hank0/positive_toolbox
#
# 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
# 第 3 版（僅此版本）重新發佈及/或修改本程式。
#
# 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
# 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
#
# 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
# 如果沒有，請參見 <https://www.gnu.org/licenses/>。

import os
import subprocess
import sys


def main():
    if (
        hasattr(sys, "frozen") is True
        and getattr(sys, "frozen") is True
    ) and (hasattr(sys, "_MEIPASS") is True):
        main_exec_path = os.path.join(
            os.path.dirname(sys.executable), "positive_toolbox"
        )
        file_base_path = os.path.dirname(sys.executable)
    else:
        print("這是為打包後檔案結構設計的", file=sys.stderr)
        main_exec_path = os.path.join(
            os.path.dirname(__file__), "positive_toolbox"
        )
        file_base_path = os.path.dirname(__file__)
    if os.name == "nt":
        main_exec_path = main_exec_path + ".exe"
        #
    print(main_exec_path)
    file_err = open(
        os.path.join(file_base_path, ".stderr.txt"),
        "a",
        encoding="utf-8",
    )
    file_out = open(
        os.path.join(file_base_path, ".stdout.txt"),
        "a",
        encoding="utf-8",
    )
    file_in = open(
        os.path.join(file_base_path, ".stdin.txt"),
        "a",
        encoding="utf-8",
    )
    process = subprocess.Popen(
        [main_exec_path],
        stdout=file_out,
        stdin=file_in,
        stderr=file_err,
        creationflags=0x08000000,
    )
    if process.wait() != 0:
        print("發生錯誤", file=sys.stderr)
    #
    file_err.close()


if __name__ == "__main__":
    main()
