#!/usr/bin/env bash

docker run \
       --env MATURIN_PASSWORD="$MATURIN_PASSWORD" \
       --env PATH=/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/root/.cargo/bin:/opt/rh/devtoolset-2/root/usr/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
       --rm \
       -v "$(pwd)":/io \
       konstin2/maturin:master \
       publish \
       --interpreter python3.7 python3.8 \
       --username __token__ \
       --password "$MATURIN_PASSWORD"
