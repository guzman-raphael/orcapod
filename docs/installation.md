# Installation

At this stage, Orcapod is available for development use through a VS Code dev container.  
This ensures a reproducible environment with Docker and Jupyter preconfigured.  

---

## Prerequisites

### Docker

Install instructions:

- **macOS & Windows:** Install [Docker Desktop](https://www.docker.com/products/docker-desktop).

- **Windows (requires WSL 2):**
    1. Open **PowerShell (Admin)** and run:

        ```bash
           wsl --install
        ```

        Reboot if prompted.

    2. Ensure WSL 2 is the default:

        ```bash
            wsl --set-default-version 2
        ```

    3. Open **Docker Desktop** once to finish setup. In *Settings → General*, confirm **Use the WSL 2 based engine** is checked.

- **Linux:** Install [Docker Engine](https://docs.docker.com/engine/install/)

### Verify Docker

```bash
docker --version
docker run --rm hello-world
```

### Visual Studio Code and Extension

- [VS Code](https://code.visualstudio.com/)
- [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- [Jupyter](https://marketplace.visualstudio.com/items?itemName=ms-toolsai.jupyter)

## Clone the repository

```bash
git clone https://github.com/guzman-raphael/orcapod
cd orcapod
```

## Open in the Dev Container

1. Open the `orcapod` folder in VS Code.
2. When prompted, click **Reopen in Container** (or run **Dev Containers: Reopen in Container** from the Command Palette).
3. VS Code will build/attach the container using .devcontainer
4. Make sure Docker Desktop/Engine is running before opening the folder.
