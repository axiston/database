# Makefile for SeaORM Migration and Entity Generation

# Directories
CONNECT_DIR=./crates/connect
MIGRATE_DIR=./crates/migrate
ENTITY_OUTPUT_DIR=./entity

# Commands
SEA_ORM_CLI=sea-orm-cli

# Environment Variables
DATABASE_URL=$(DATABASE_URL)
DATABASE_SCHEMA=$(or $(DATABASE_SCHEMA),public)

# https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/
.PHONY: create
create:
	@echo "Generating a new migration..."
	@read -p "Enter migration name: " name; \
	cd $(MIGRATE_DIR) && $(SEA_ORM_CLI) migrate generate "$$name"; \
	echo "Migration '$$name' created successfully."

# https://www.sea-ql.org/SeaORM/docs/migration/running-migration/
.PHONY: migrate
migrate:
	@echo "Running migrations..."
	cd $(MIGRATE_DIR) && $(SEA_ORM_CLI) migrate up
	@echo "Migrations executed successfully."

# https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/
.PHONY: generate
generate:
	@echo "Regenerating entities..."
	cd $(CONNECT_DIR) && \
	$(SEA_ORM_CLI) generate entity \
		--database-url "$(DATABASE_URL)" \
		--database-schema "$(DATABASE_SCHEMA)" \
		--output-dir "$(ENTITY_OUTPUT_DIR)" \
		--date-time-crate "time" \
		--with-serde "both" \
		--with-copy-enums \
		--expanded-format \
		--verbose
	@echo "Entities regenerated successfully."
