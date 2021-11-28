FROM php:7.4.2

# RUN apt-get update && apt install zip -y
# RUN apt-get clean autoclean && apt-get autoremove -y && rm -rf /var/lib/{apt,dpkg,cache,log}/

COPY --from=composer /usr/bin/composer /usr/bin/composer

WORKDIR /usr/src/app

COPY src/* ./

RUN apt-get update
RUN apt-get -y install zip
RUN composer install

ENTRYPOINT [ "php" "/usr/src/app/slack.php" ]