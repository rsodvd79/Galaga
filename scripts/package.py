#!/usr/bin/env python3
"""Package a native build without committing generated binaries."""
import argparse
import pathlib
import plistlib
import shutil
import tempfile
import zipfile


def package(target, version):
    root = pathlib.Path(__file__).resolve().parents[1]
    windows = "windows" in target
    macos = "apple" in target
    binary = root / "target" / target / "release" / ("galaga.exe" if windows else "galaga")
    if not binary.is_file():
        raise FileNotFoundError(binary)
    output = root / "publish" / f"Galaga-{version}-{target}.zip"
    output.parent.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory() as temporary:
        staging = pathlib.Path(temporary)
        if macos:
            contents = staging / "Galaga.app" / "Contents"
            executable_dir = contents / "MacOS"
            resources = contents / "Resources"
            executable_dir.mkdir(parents=True)
            resources.mkdir()
            shutil.copy2(binary, executable_dir / "Galaga")
            shutil.copy2(root / "assets" / "icon.icns", resources / "Galaga.icns")
            with (root / "packaging" / "macos" / "Info.plist").open("rb") as file:
                info = plistlib.load(file)
            info["CFBundleIdentifier"] = "com.galaga.rust"
            info["CFBundleVersion"] = version
            info["CFBundleShortVersionString"] = version
            with (contents / "Info.plist").open("wb") as file:
                plistlib.dump(info, file)
        else:
            shutil.copy2(binary, staging / binary.name)
        shutil.copy2(root / "README.md", staging / "README.md")
        with zipfile.ZipFile(output, "w", zipfile.ZIP_DEFLATED) as archive:
            for path in sorted(staging.rglob("*")):
                if path.is_file():
                    archive.write(path, path.relative_to(staging))
        with zipfile.ZipFile(output) as archive:
            assert archive.testzip() is None
            expected = "Galaga.app/Contents/MacOS/Galaga" if macos else binary.name
            assert expected in archive.namelist()
    print(output)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("target")
    parser.add_argument("version")
    args = parser.parse_args()
    package(args.target, args.version)
