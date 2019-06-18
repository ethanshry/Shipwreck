# Shipwreck
Host and Deployment Service

## MQ

Shipwreck is both a Publisher and Consumer of RabbitMQ, and requires a running server to function.
Start a server with `sudo systemctl start rabbitmq-server.service`
and stop it with `sudo systemctl enable rabbitmq-server.service`

Its status can be checked with `sudo rabbitmqctl status`

Installed following steps [here](https://www.vultr.com/docs/how-to-install-rabbitmq-on-ubuntu-16-04-47)
