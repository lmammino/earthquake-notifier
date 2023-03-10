# earthquake-notifier

WORK in progress Lambda in Rust to monitor for earthquakes and generate EventBridge events.

Under the hood, it uses [INGV APIs](http://webservices.ingv.it/swagger-ui/dist/?url=https://ingv.github.io/openapi/fdsnws/event/0.0.1/event.yaml) and scans them every hour.