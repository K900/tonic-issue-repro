#!/bin/sh
echo "- Starting proxy..."
/proxy &
sleep 1

echo "- Making request..."
curl http://127.0.0.1:8080/hello

echo "- Starting server..."
/server &
sleep 1

echo "- Making request..."
curl http://127.0.0.1:8080/hello

echo "- Restarting proxy..."
pkill -9 proxy
/proxy &
sleep 1

echo "- Making request..."
curl http://127.0.0.1:8080/hello
