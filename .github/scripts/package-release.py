"""Package a tested native executable and its license notices for GitHub Releases."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import tomllib
import zipfile


ROOT = Path(__file__).resolve().parents[2]


def dependency_notices(target):
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--format-version", "1",
        "--filter-platform", target,
    ], cwd=ROOT))
    packages = {package["id"]: package for package in metadata["packages"]}
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    root_id = next(key for key, package in packages.items()
                   if Path(package["manifest_path"]).resolve() == ROOT / "Cargo.toml")
    selected = set()
    pending = [root_id]
    while pending:
        current = pending.pop()
        if current in selected:
            continue
        selected.add(current)
        pending.extend(dependency["pkg"] for dependency in nodes[current]["deps"]
                       if any(kind["kind"] is None for kind in dependency["dep_kinds"]))

    sections = ["License and notice files supplied with rdstream's dependencies.\n"]
    for key in sorted(selected - {root_id}, key=lambda key: packages[key]["name"]):
        package = packages[key]
        directory = Path(package["manifest_path"]).parent
        notices = sorted(path for path in directory.rglob("*")
                         if path.is_file() and path.name.upper().startswith(
                             ("LICENSE", "LICENCE", "COPYING", "NOTICE")))
        supplemental = ROOT / ".github/licenses" / f"{package['name']}-{package['version']}.txt"
        if supplemental.is_file():
            notices.append(supplemental)
        if not notices:
            raise RuntimeError(f"No license notice found for {package['name']}")
        sections.append(f"\n{'=' * 72}\n{package['name']} {package['version']}\n"
                        f"License expression: {package.get('license', 'see notices')}\n")
        for notice in notices:
            label = notice.relative_to(directory) if notice.is_relative_to(directory) else notice.name
            sections.append(f"\n--- {label} ---\n")
            sections.append(notice.read_text(encoding="utf-8", errors="replace"))
    return "\n".join(sections)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target")
    parser.add_argument("--check-version", action="store_true")
    args = parser.parse_args()
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]["version"]
    tag = os.environ["RELEASE_TAG"]
    if tag != f"v{version}":
        raise SystemExit(f"Release tag {tag!r} does not match package version {version!r}")
    if args.check_version:
        print(f"Verified release version: {tag}")
        return
    if not args.target:
        parser.error("--target is required when packaging")

    executable_name = "rdstream.exe" if "windows" in args.target else "rdstream"
    executable = ROOT / "target" / args.target / "release" / executable_name
    actual = subprocess.check_output([str(executable), "--version"], text=True).strip()
    if actual != f"rdstream {version}":
        raise SystemExit(f"Unexpected executable version: {actual!r}")
    subprocess.run([
        str(executable), "describe",
        str(ROOT / "tests/fixtures/rdata/generated/test_complex__xdr__version_3.rds"),
        "--depth", "2",
    ], check=True)

    name = f"rdstream-{tag}-{args.target}"
    output = ROOT / "dist"
    output.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="rdstream-release-") as temporary:
        directory = Path(temporary) / name
        directory.mkdir()
        shutil.copy2(executable, directory / executable_name)
        for filename in ["LICENSE-MIT", "LICENSE-APACHE"]:
            shutil.copy2(ROOT / filename, directory / filename)
        (directory / "README.txt").write_text(
            f"rdstream {version}\nTarget: {args.target}\n\n"
            "Add the executable to a directory on PATH, then run:\n\n"
            "  rdstream --help\n"
            "  rdstream describe dataset.rds --depth 4\n\n"
            "Documentation: https://github.com/HiroGitea/rdstream\n"
            "License: MIT OR Apache-2.0; see the included license files.\n",
            encoding="utf-8")
        (directory / "THIRD-PARTY-NOTICES.txt").write_text(
            dependency_notices(args.target), encoding="utf-8")
        if "windows" in args.target:
            archive = output / f"{name}.zip"
            with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
                for path in sorted(directory.iterdir()):
                    bundle.write(path, arcname=f"{name}/{path.name}")
        else:
            archive = output / f"{name}.tar.gz"
            with tarfile.open(archive, "w:gz") as bundle:
                bundle.add(directory, arcname=name)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    (output / f"{archive.name}.sha256").write_text(
        f"{checksum}  {archive.name}\n", encoding="ascii")
    print(f"Created {archive.name} ({archive.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
