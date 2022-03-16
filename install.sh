SERVICE_NAME="ruleset-randomizer"
SERVICE_PATH="/lib/systemd/system/$SERVICE_NAME.service"
cp $SERVICE_NAME.service $SERVICE_PATH
cp "etc/nginx/conf.d/$SERVICE_NAME.conf" "/etc/nginx/conf.d/$SERVICE_NAME.conf"
