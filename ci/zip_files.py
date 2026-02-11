import tomllib
import zipfile
import os
import platform


def main():
    info_file = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), "Cargo.toml"
    )
    with open(info_file, "rb", encoding="utf-8") as f:
        project_info = tomllib.load(f)
    version = project_info["package"]["version"]
    #
    include_files = []
    target_path = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        "target",
        "release",
    )
    for file in os.listdir(target_path):
        if (
            os.path.splitext(file)[1] == "exe"
            or os.path.splitext(file)[1] == ""
        ):
            include_files.append(os.path.join(target_path, file))
    #
    match platform.platform():
        case "Linux":
            pf = "linux"
        case "Windows":
            pf = "windows"
        case _:
            pf = "unknown"
    with zipfile.ZipFile(
        f"positive_toolbox_{version}_{pf}",
        mode="w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zipf:
        for file in include_files:
            zipf.write(file, arcname=os.path.basename(file))
    print(f"positive_toolbox_{version}_{pf}")


if __name__ == "__main__":
    main()
