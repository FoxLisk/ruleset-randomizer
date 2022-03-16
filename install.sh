SERVICE_NAME="ruleset_randomizer"
SERVICE_PATH="/lib/systemd/system/ruleset_randomizer.service"
cp "target/debug/$SERVICE_NAME" "/usr/bin/$SERVICE_NAME"
if test -f "$SERVICE_PATH";
then
  :
else
  cp $SERVICE_NAME.service $SERVICE_PATH
fi

