import tomllib
import zipfile
import os
import platform


def main():
    info_file = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), "Cargo.toml"
    )
    with open(info_file, "rb") as f:
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
        full_file_path = os.path.join(target_path, file)
        if os.path.isfile(full_file_path) is True:
            match platform.system():
                case "Linux":
                    if (
                        os.path.splitext(full_file_path)[1] == ""
                    ) and (
                        len(os.path.splitext(full_file_path)[1]) > 1
                    ):
                        include_files.append(full_file_path)
                case "Windows":
                    if (
                        os.path.splitext(full_file_path)[1] == "exe"
                    ) and (
                        len(os.path.splitext(full_file_path)[1]) > 1
                    ):
                        include_files.append(full_file_path)
    #
    match platform.system():
        case "Linux":
            pf = "linux"
        case "Windows":
            pf = "windows"
        case _:
            pf = "unknown"
    with zipfile.ZipFile(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            f"positive_toolbox_{version}_{pf}.zip",
        ),
        mode="w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zipf:
        for file in include_files:
            zipf.write(file, arcname=os.path.basename(file))
    print(f"positive_toolbox_{version}_{pf}.zip")


if __name__ == "__main__":
    main()
