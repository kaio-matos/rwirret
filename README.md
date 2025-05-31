# Rwirret

## Installation

```
docker compose up -d
```

```
docker compose exec app dx serve --platform web
```

Generate new empty migration

```
docker compose exec app sea-orm-cli migrate generate create_post_table
```

Run migrations

```
docker compose exec app sea-orm-cli migrate up
```

Generate entities from the migrations

```
docker compose exec app sea-orm-cli generate entity -o entity/src
```
