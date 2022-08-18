json2ts -i ../contracts/cw20_hub/schema -o ./types/cw20_hub/
json2ts -i ../contracts/osmosis_hub/schema -o ./types/osmosis_hub/

for f in ./types/**/*.d.ts; do
  mv -- "$f" "${f%.d.ts}.ts"
done