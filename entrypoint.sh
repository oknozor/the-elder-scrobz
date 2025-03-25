#!/bin/sh

# Run migrations before starting the application
echo "Running database migrations..."
sqlx migrate run --source /app/migrations

# Execute the main command (the application)
echo "Starting application..."
exec "$@"
