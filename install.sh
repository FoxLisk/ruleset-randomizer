set -e
SERVICE_NAME="ruleset-randomizer"
SERVICE_PATH="/lib/systemd/system/$SERVICE_NAME.service"
cargo build
npx tailwindcss -i ./build_templates/index.css -o ./static/index.css
sudo cp $SERVICE_NAME.service $SERVICE_PATH
sudo cp "etc/nginx/conf.d/$SERVICE_NAME.conf" "/etc/nginx/conf.d/$SERVICE_NAME.conf"
sudo systemctl daemon-reload
sudo systemctl restart "$SERVICE_NAME"
sudo systemctl enable "$SERVICE_NAME"
