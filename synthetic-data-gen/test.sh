#!/bin/bash

curl -s "http://localhost:3000/generate?fields=first_name,last_name,email,dob,ssn&count=2" | jq .