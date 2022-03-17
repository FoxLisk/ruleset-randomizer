# Tailwind CSS

get npm installed (i used nvm. on WSL for windows.)

follow install steps (from https://tailwindcss.com/docs/installation)

1. npm install -D tailwindcss
1. npx tailwindcss init

now you have a tailwind.config.js. make sure it has relevant files listed, and then run

    npx tailwindcss -i ./build_templates/index.css -o ./static/index.css
    
(this has been added to install.sh)

you can run it with `--watch` to rebuild live, but that seems like kind of a shitty way to live. i might just make the
dev version with the <script> tag live in dev.

# TODO

make techniques into a richer data structure, so I can generate valid rulesets and such from it