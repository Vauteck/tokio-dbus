docker run --rm -it -u `id -u`:`id -g` -w $(pwd) -v $(pwd):$(pwd) rootfs-docker-x64 /bin/bash
