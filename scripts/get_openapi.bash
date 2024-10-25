#!/bin/bash
cd ..

url=https://api.app.reclaim.ai/swagger/reclaim-api-0.1.yml
echo "Fetching swagger from $url"

curl "$url" --max-time 5 > api.yaml