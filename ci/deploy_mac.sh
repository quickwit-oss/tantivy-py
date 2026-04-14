#!/usr/bin/env bash

maturin publish \
    --interpreter python3.10 \
    --username __token__ \
    --password "$MATURIN_PASSWORD" \
    --no-sdist
