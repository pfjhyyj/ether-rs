# Ether-RS

A template backend project written by rust

## Common commands

```bash
# first set DATABASE_URL in .env
# migrate db
sea-orm-cli migrate up
# generate entities
sea-orm-cli generate entity -o entity/src -l
```
