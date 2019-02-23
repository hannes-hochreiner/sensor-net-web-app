npm run build
rsync -arvz -e "ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" --rsync-path="sudo rsync" --delete-after --progress ./dist/ hc2:/opt/sensor-net-web-app
rsync -arvz -e "ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" --rsync-path="sudo rsync" --progress ./etc_nginx/ hc2:/etc/nginx/sites-available
ssh hc2 "sudo rm /etc/nginx/sites-enabled/*"
ssh hc2 "sudo ln -s /etc/nginx/sites-available/sensor-net-web-app-site /etc/nginx/sites-enabled/sensor-net-web-app-site"
ssh hc2 "sudo systemctl restart nginx"
