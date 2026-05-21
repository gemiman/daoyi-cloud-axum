sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  -s demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/demo/entity

# sea-orm-cli generate entity \
  #  -u mysql://root:123456@127.0.0.1:3306/demo \
  #  --with-serde both \
  #  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  #  --date-time-crate chrono \
  #  -o ./src/demo/entity \
  #  --ignore-tables demo_contact \
  #  --ignore-tables demo_category \
  #  --ignore-tables demo_course \
  #  --ignore-tables demo_grade \
  #  --ignore-tables demo_student