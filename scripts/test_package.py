import pathlib
import plistlib
import tempfile
import unittest
import zipfile
from unittest.mock import patch

import package


class PackageTests(unittest.TestCase):
    def test_archive_names_and_native_contents(self):
        version = "2026.10.31.22.14"
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "README.md").write_text("Galaga", encoding="utf-8")
            (root / "assets").mkdir()
            (root / "assets" / "icon.icns").write_bytes(b"icon")
            (root / "packaging" / "macos").mkdir(parents=True)
            with (root / "packaging" / "macos" / "Info.plist").open("wb") as file:
                plistlib.dump({}, file)
            targets = [
                ("x86_64-unknown-linux-gnu", "linux", "galaga"),
                ("x86_64-pc-windows-msvc", "windows", "galaga.exe"),
                ("x86_64-apple-darwin", "macos-intel", "galaga"),
                ("aarch64-apple-darwin", "macos-arm64", "galaga"),
            ]
            for target, os_name, binary_name in targets:
                with self.subTest(target=target):
                    binary = root / "target" / target / "release" / binary_name
                    binary.parent.mkdir(parents=True)
                    binary.write_bytes(target.encode())
                    with patch.object(package, "__file__", str(root / "scripts" / "package.py")):
                        package.package(target, version)
                    with zipfile.ZipFile(root / "publish" / f"Galaga.{os_name}.{version}.zip") as archive:
                        executable = (
                            "Galaga.app/Contents/MacOS/Galaga"
                            if os_name.startswith("macos")
                            else binary_name
                        )
                        self.assertEqual(archive.read(executable), target.encode())
                        self.assertEqual(archive.read("README.md"), b"Galaga")
            self.assertEqual(len(list((root / "publish").glob("*.zip"))), len(targets))


if __name__ == "__main__":
    unittest.main()
