#!/usr/bin/env bash

docker run \
       --env MATURIN_PASSWORD="$MATURIN_PASSWORD" \
       --rm \
       -v "$(pwd)":/io \
       konstin2/maturin \
       publish \
       --interpreter python3.7 python3.8 \
       --username __token__ \
       --password "$MATURIN_PASSWORD"
