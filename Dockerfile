FROM yangfh2004/sgx-rust:22.04-1.76-nightly AS builder
# See label assignment pattern: https://forums.docker.com/t/tag-intermediate-build-stages-multi-stage-build/34795/6
LABEL xyz.dexlabs.tag="async-runtime"
LABEL version="0.1.0"
ARG SGX_PRERELEASE
ARG SGX_MODE
ARG JOBS
ARG DEBUG_MODE
ENV LD_LIBRARY_PATH="/lib/x86_64-linux-gnu:/opt/intel/sgx-aesm-service/aesm:${LD_LIBRARY_PATH}" \
    TEACLAVE_SDK="/opt/teaclave/rust-sgx-sdk" \
    CERTS_DIR="/usr/local/src/certs/" \
    SGX_MODE="${SGX_MODE}" \
    SGX_PRERELEASE="${SGX_PRERELEASE}" \
    RUST_LOG="info" \
    RUST_BACKTRACE="full" \
    DEBUG_MODE="${DEBUG_MODE}" \
    SGX_DEBUG="${DEBUG_MODE}" \
    JOBS="${JOBS}" \
    CFLAGS="-DRING_CORE_NOSTDLIBINC=1" \
    LANG="C.UTF-8" \
    REMOTE_URL="https://gitlab.com/dexlabs/incubator-teaclave-sgx-sdk" \
    BRANCH_NAME="v2.0.0-sgx-emm"
# hadolint ignore=DL3008
RUN bash -c "echo \"PS1='${debian_chroot:+($debian_chroot)}\[\033[01;31m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '\" >> ~/.bashrc " \
    && mkdir -p /var/local/ddx /var/local/log /var/local/cargo \
    && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc |apt-key add - \
    && echo "deb http://apt.postgresql.org/pub/repos/apt/ jammy-pgdg main" |tee /etc/apt/sources.list.d/pgdg.list \
    && apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    postgresql-client-13 \
    libssl-dev libffi-dev \
    libpq-dev \
    iproute2 \
    rsync \
    curl \
    git \
    jq \
    gettext \
    ripgrep \
    vim \
    fd-find \
    lld \
    && rm -rf /var/lib/apt/lists/* \
    && ln -s /usr/bin/fdfind /usr/local/bin/fd
COPY . /usr/local/src
RUN mkdir -p ${TEACLAVE_SDK} \
    && git clone --depth 1 --branch "${BRANCH_NAME}" "${REMOTE_URL}" "${TEACLAVE_SDK}"
COPY ./entrypoint/shell-hook.bash /usr/local/bin/shell-hook.bash
WORKDIR /usr/local/src
ENTRYPOINT ["/usr/local/bin/shell-hook.bash"]
