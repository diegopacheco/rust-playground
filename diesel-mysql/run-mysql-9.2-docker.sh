#!/bin/bash

docker run --rm -d -e MYSQL_ROOT_PASSWORD=pass -p 3325:3306 --name mysql_test mysql:9.2
echo "Connect to mysql client: "
echo "mysql -uroot -ppass -h127.0.0.1 -P3325"

sleep 10
echo "Creating person db... "
echo "CREATE DATABASE person;" | mysql -uroot -ppass -h127.0.0.1 -P3325

sleep 3
echo "Creating person table... "
echo "use person; CREATE TABLE IF NOT EXISTS person (
          id INT AUTO_INCREMENT PRIMARY KEY,
          first_name VARCHAR(255) NOT NULL,
          last_name VARCHAR(255) NOT NULL,
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      ) ENGINE=INNODB;;" | mysql -uroot -ppass -h127.0.0.1 -P3325

echo "use person; select * from  person" | mysql -uroot -ppass -h127.0.0.1 -P3325
echo "DONE."