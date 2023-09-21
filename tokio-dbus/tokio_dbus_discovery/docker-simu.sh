#!/bin/bash

docker run --cap-add=SYS_PTRACE --security-opt seccomp=unconfined --rm -it -v $(pwd):$(pwd) -w $(pwd) rootfs-docker-x64 /bin/bash
