# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080

## Add tailwind dependency

```bash
npm init
npm install -D tailwindcss
npm install -D @tailwindcss/typography
npm install -D @tailwindcss/forms
yarn
yarn watch
```

```bash
dx serve --port 9090
```

## Used tailwind components

https://www.creative-tim.com/twcomponents/component/inbox
