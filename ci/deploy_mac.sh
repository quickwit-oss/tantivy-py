#!/usr/bin/env bash

maturin publish \
    --interpreter python3.7 python3.8 \
    --username __token__ \
    --password "$MATURIN_PASSWORD" \
    --no-sdist
