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
    if (hasattr(sys, "frozen") is True and getattr(sys, "frozen") is True) and (
        hasattr(sys, "_MEIPASS") is True
    ):
        main_exec_path = os.path.join(
            os.path.dirname(sys.executable), "positive_toolbox"
        )
        file_base_path = os.path.dirname(sys.executable)
    else:
        print("WARNING：這是為打包後檔案結構設計的！", file=sys.stderr)
        main_exec_path = os.path.join(
            os.path.dirname(__file__), "positive_toolbox"
        )
        file_base_path = os.path.dirname(__file__)
    if os.name == "nt":
        main_exec_path = main_exec_path + ".exe"
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
        process_stdout = str(
            "無法取得stdout！"
            if process.stdout.read().decode() is None  # pyright: ignore[reportOptionalMemberAccess]
            else process.stdout
        )
        process_stderr = str(
            "無法取得stderr！"
            if process.stderr.read().decode() is None  # pyright: ignore[reportOptionalMemberAccess]
            else process.stderr
        )
        show_error(process_stdout, process_stderr)
    #
    file_err.close()
    file_out.close()
    file_in.close()


def show_error(stdout: str, stderr: str):
    import tkinter as tk
    from tkinter import messagebox

    class CopyableTextWindow:
        def __init__(self, root: tk.Tk) -> None:
            self.root: tk.Tk = root
            self.text_content: str = "文字"
            self.setup_ui()

        def setup_ui(self) -> None:
            self.root.title("positive_toolbox launcher - error-handler")
            self.root.geometry("540x360")

            # 建立文字標籤
            self.label: tk.Label = tk.Label(
                self.root,
                text=self.text_content,
                font=("Microsoft JhengHei", 12),
                wraplength=300,
                justify=tk.CENTER,
            )
            self.label.pack(pady=20)

            # 建立按鈕框架
            self.btn_frame: tk.Frame = tk.Frame(self.root)
            self.btn_frame.pack()

            # 建立複製按鈕
            self.copy_btn: tk.Button = tk.Button(
                self.btn_frame,
                text="複製文字",
                command=self.copy_to_clipboard,
                width=10,
            )
            self.copy_btn.pack(side=tk.LEFT, padx=10)

            # 建立關閉按鈕
            self.close_btn: tk.Button = tk.Button(
                self.btn_frame,
                text="關閉視窗",
                command=self.root.destroy,
                width=10,
            )
            self.close_btn.pack(side=tk.LEFT, padx=10)

        def copy_to_clipboard(self) -> None:
            try:
                self.root.clipboard_clear()
                self.root.clipboard_append(self.text_content)
                self.root.update()  # 確保剪貼簿內容已更新
                messagebox.showinfo("成功", "文字已複製到剪貼簿")
            except Exception as e:
                messagebox.showerror("錯誤", f"複製失敗：{str(e)}")

    root: tk.Tk = tk.Tk()
    # 設定 DPI 感知（針對高解析度螢幕）
    try:
        from ctypes import windll

        windll.shcore.SetProcessDpiAwareness(1)
    except Exception:
        pass

    app: CopyableTextWindow = CopyableTextWindow(root)
    app.text_content = (
        f"--- stdout --- \n{stdout} \n\n --- stderr --- \n {stderr}"
    )
    root.mainloop()


if __name__ == "__main__":
    main()
