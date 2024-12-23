# Makefile for Diesel Migration and Entity Generation.
# https://diesel.rs/guides/getting-started

# Environment Variables:
POSTGRES_HOST ?= localhost
POSTGRES_PORT ?= 5432
POSTGRES_USERNAME ?= postgres
POSTGRES_PASSWORD ?= postgres
POSTGRES_DATABASE ?= postgres
SCHEMA_OUTPUT = ./crates/schema/schema.rs

# Construct database address using environment variables.
DATABASE_URL = postgresql://$(POSTGRES_USERNAME):$(POSTGRES_PASSWORD)@${POSTGRES_HOST}:${POSTGRES_PORT}/$(POSTGRES_DATABASE)

all: migrate

.PHONY: install
install: ## Installs the Diesel CLI.
	$(call print-info, "Installing Diesel CLI...")
	cargo install diesel_cli --no-default-features --features postgres
	$(call print-success, "Diesel CLI installed successfully.")

.PHONY: migrate
migrate: ## Runs all Postgres migrations.
	$(call print-info, "Running migrations...")
	DATABASE_URL=$(DATABASE_URL) diesel migration run
	$(call print-success, "Migrations applied successfully.")

.PHONY: rollback
rollback: ## Rolls back the last migration.
	$(call print-info, "Rolling back last migration...")
	DATABASE_URL=$(DATABASE_URL) diesel migration revert
	$(call print-success, "Migration rolled back successfully.")

.PHONY: clean
clean: ## Deletes the output schema file.
	$(call print-info, "Cleaning project...")
	rm -f $(SCHEMA_OUTPUT)
	$(call print-success, "Project cleaned.")

.PHONY: generate
generate: ## Generates and updates the schema file.
	$(call print-info, "Printing database schema...")
	DATABASE_URL=$(DATABASE_URL) diesel print-schema > $(SCHEMA_OUTPUT)
	$(call print-success, "Schema updated successfully in $(SCHEMA_OUTPUT)")
