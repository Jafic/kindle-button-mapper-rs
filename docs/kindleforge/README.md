# KindleForge submission

`install.sh` here points at the GitHub Releases artifact published by `.github/workflows/build-arm.yml` on every `v*` tag push. Cut a release first:

```bash
git tag v0.1.0
git push origin v0.1.0
# CI builds + publishes kindle-button-mapper-armv7.tar.gz to GitHub Releases
```

Then open the PR against https://github.com/KindleTweaks/KindleForge:

```bash
git clone git@github.com:YOUR_USER/KindleForge.git   # your fork
cd KindleForge
cp -r /path/to/kindle-button-mapper-rs/docs/kindleforge/KindleButtonMapper \
  Repository/KindleButtonMapper

# insert this entry before the closing ] of Repository/registry.json:
```

```json
    {
        "name": "Kindle Button Mapper",
        "uri": "KindleButtonMapper",
        "description": "Map gamepad/remote/keyboard buttons to KOReader, key events, or custom scripts",
        "author": "Lucas Zampieri",
        "ABI": ["hf", "sf"],
        "dependencies": [],
        "tags": ["UTILITY"]
    }
```

```bash
git checkout -b add-kindle-button-mapper
git add Repository/KindleButtonMapper Repository/registry.json
git commit -s -m "Add Kindle Button Mapper package"
git push origin add-kindle-button-mapper
# open PR on github.com
```

## Pre-PR checklist

- [ ] Tag pushed and CI release succeeded (tarball at `releases/latest/download/kindle-button-mapper-armv7.tar.gz`)
- [ ] `install.sh` runs cleanly on a fresh Kindle
- [ ] `uninstall.sh` cleans up fully
