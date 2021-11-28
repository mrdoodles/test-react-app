FROM php:7.4.2

RUN apt-get update && apt install zip -y
RUN apt-get clean autoclean && apt-get autoremove -y && rm -rf /var/lib/{apt,dpkg,cache,log}/

COPY --from=composer /usr/bin/composer /usr/bin/composer

WORKDIR /usr/src/app

COPY src/* ./

RUN composer install

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT [ "php" "/usr/src/app/slack.php" ]