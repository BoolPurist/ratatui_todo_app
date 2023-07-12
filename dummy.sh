#!/bin/sh
set -e

end_number=100

last=$((end_number - 1))
echo "["
for number in $(seq 1 $last)
do
  echo "    {"
  echo "        \"content\": \"$number\","
  echo "        \"done\": true"
  echo "    },"
done


  echo "    {"
  echo "        \"content\": \"$end_number\","
  echo "        \"done\": true"
  echo "    }"

echo "]"
