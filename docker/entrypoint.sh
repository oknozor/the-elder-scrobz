#!/bin/sh

echo "Generating frontend environment configuration..."

cat > /app/frontend/config.js << EOF
window.env = {
  VITE_OIDC_AUTHORITY: "${SCROBZ_OPENID_AUTHORITY}",
  VITE_OIDC_CLIENT_ID: "${SCROBZ_OPENID_CLIENT_ID}",
  VITE_OIDC_CLIENT_SECRET: "${SCROBZ_OPENID_CLIENT_SECRET}"
};
EOF

echo "Frontend environment configuration generated at /app/frontend/config.js"

echo "Starting application..."

exec "$@"
