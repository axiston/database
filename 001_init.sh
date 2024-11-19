#!/usr/bin/env bash

# Read the database name from the environment variable.
# Default to 'postgres' if $POSTGRES_DB is not set.
dbname="${POSTGRES_DB:-postgres}"
customconf=/var/lib/postgresql/data/axiston.conf
mainconf=/var/lib/postgresql/data/postgresql.conf

# Create or overwrite the custom configuration file.
cat > "$customconf" <<EOF
shared_preload_libraries = 'pg_cron'
cron.database_name = '$dbname'
EOF

chown postgres $customconf
chgrp postgres $customconf

# Include the custom config file from main config file.
if ! grep -q "include = '$customconf'" "$mainconf"; then
  echo "include = '$customconf'" >> "$mainconf"
fi

pg_ctl restart
