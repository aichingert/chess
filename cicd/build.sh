set -e

mkdir -p dist

wasm-pack build --target web --out-dir dist

echo "<!DOCTYPE html>
<html lang=\"en\">

<head>
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Learn WGPU</title>
    <style>
        canvas {
            background-color: white;
        }

	body {
	    display: flex;
	    align-items: center;
	    justify-content: center;
	    height: 100%;
	}
    </style>
</head>

<body id=\"wasm-example\">
  <script type=\"module\">
      import init from \"./chess.js\";
      init().then(() => {
          console.log(\"WASM Loaded\");
      });
  </script>
</body>

</html>" > dist/index.html
