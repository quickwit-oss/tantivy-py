#!/usr/bin/env bash

docker run \
       --env MATURIN_PASSWORD="$MATURIN_PASSWORD" \
       --rm \
       -v "$(pwd)":/io \
       konstin2/maturin \
       publish \
       --username __token__ \
       --password "$MATURIN_PASSWORD"
