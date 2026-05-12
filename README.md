# image-library

Docker images for the spinupdev platform. Images are published to
[ghcr.io/spinupdev](https://github.com/orgs/spinupdev/packages).

## Images

| Image | Base | Description |
|-------|------|-------------|
| [`desktop`](images/desktop) | ubuntu:26.04 | XFCE4 desktop over VNC/noVNC, Firefox, supervisord |
| [`ubuntu`](images/ubuntu) | ubuntu:26.04 | Base Ubuntu with SSH, user setup, and init |
| [`workstation`](images/workstation) | ubuntu:26.04 | Dev workstation with Docker, Go, Node 26, Python, AI agent CLIs |

## Building locally

Each image has a `Makefile` with standard targets.

```sh
# Build a specific image
make -C images/desktop build

# Build and run desktop (opens on :8080)
make -C images/desktop run
```

Or use Docker Compose for the desktop image:

```sh
cd images/desktop
docker compose up
```

Environment variables for the desktop image:

| Variable | Default | Description |
|----------|---------|-------------|
| `VNC_PASSWORD` | `changeme` | VNC password |
| `VNC_RESOLUTION` | `1280x720` | Display resolution |
| `VNC_DEPTH` | `24` | Color depth |
| `AUTH_ENABLED` | `false` | Enable JWT auth on the noVNC proxy |

## Releasing

Releases are tag-driven. Pushing a tag of the form `<image>/v<semver>` triggers
the [build workflow](.github/workflows/build.yml), which builds the image and
pushes both the version tag and `latest` to GHCR.

```sh
# Release desktop v1.2.0
git tag desktop/v1.2.0
git push origin desktop/v1.2.0

# Release ubuntu v1.0.0
git tag ubuntu/v1.0.0
git push origin ubuntu/v1.0.0
```

The workflow builds for all platforms listed in the image's `platform` file.

## Adding a new image

1. Create the image directory and Dockerfile:
   ```
   images/<name>/Dockerfile
   ```

2. Add a `platform` file listing the target architectures:
   ```sh
   echo "linux/amd64,linux/arm64" > images/<name>/platform
   ```

3. Tag a release to publish:
   ```sh
   git tag <name>/v1.0.0 && git push origin <name>/v1.0.0
   ```

No workflow changes needed — the build workflow picks up any image automatically
from the tag prefix.
