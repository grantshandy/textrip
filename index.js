import init, {Coords, warp_image, get_dimensions} from './wasm/wasm.js';

const canvas = new fabric.Canvas('ripper', { selection: false });

fabric.Object.prototype.originX = fabric.Object.prototype.originY = 'center';

function makeCircle(left, top, line1, line2) {
  var c = new fabric.Circle({
    left: left,
    top: top,
    strokeWidth: 2,
    radius: 6,
    fill: '#292929',
    stroke: '#eeeeec'
  });
  c.hasControls = c.hasBorders = false;

  c.line1 = line1;
  c.line2 = line2;

  return c;
}

function makeLine(coords) {
  return new fabric.Line(coords, {
    stroke: '#eeeeec',
    strokeWidth: 2,
    selectable: false,
    evented: false,
  });
}

const canvasObjects = [];
let topLeftCircle;
let topRightCircle;
let bottomLeftCircle;
let bottomRightCircle;

function addToCanvas(fabricObject) {
  canvas.add(fabricObject);
  canvasObjects.push(fabricObject);
}

function clearCanvasObjects() {
  canvasObjects.forEach(function (canvasObject) {
    canvas.remove(canvasObject);
  });
}

function initializeControls(width, height) {
  clearCanvasObjects();

  var topHeight = Math.floor(height / 4);
  var bottomHeight = Math.floor((height / 4) * 3);
  var rightWidth = Math.floor((width / 4) * 3);
  var leftWidth = Math.floor(width / 4);

  const topLine = makeLine([leftWidth, topHeight, rightWidth, topHeight]);
  const rightLine = makeLine([rightWidth, topHeight, rightWidth, bottomHeight]);
  const bottomLine = makeLine([rightWidth, bottomHeight, leftWidth, bottomHeight]);
  const leftLine = makeLine([leftWidth, bottomHeight, leftWidth, topHeight]);

  addToCanvas(topLine);
  addToCanvas(leftLine);
  addToCanvas(rightLine);
  addToCanvas(bottomLine);

  topLeftCircle = makeCircle(topLine.get('x1'), topLine.get('y1'), topLine, leftLine);
  topRightCircle = makeCircle(topLine.get('x2'), topLine.get('y2'), rightLine, topLine);
  bottomLeftCircle = makeCircle(bottomLine.get('x2'), bottomLine.get('y2'), leftLine, bottomLine);
  bottomRightCircle = makeCircle(bottomLine.get('x1'), bottomLine.get('y1'), bottomLine, rightLine);

  addToCanvas(topLeftCircle);
  addToCanvas(topRightCircle);
  addToCanvas(bottomLeftCircle);
  addToCanvas(bottomRightCircle);

  canvas.renderAll();
}

canvas.on('object:moving', function(e) {
  var p = e.target;
  p.line1 && p.line1.set({
    x1: p.left,
    y1: p.top,
  });

  p.line2 && p.line2.set({
    x2: p.left,
    y2: p.top,
  });

  canvas.renderAll();
});

function getPoints() {
  var topLeft = [topLeftCircle.left, topLeftCircle.top];
  var topRight = [topRightCircle.left, topRightCircle.top];
  var bottomLeft = [bottomLeftCircle.left, bottomLeftCircle.top];
  var bottomRight = [bottomRightCircle.left, bottomRightCircle.top];

  print_points([topLeft, topRight, bottomLeft, bottomRight]);
}

function coords(circle) {
  return new Coords(circle.left, circle.top);
}

let imageBytes = null;

(async () => {
  await init();

  initializeControls(canvas.width, canvas.height);

  var button = document.getElementById('button');

  button.addEventListener('input', (event) => {
    let file = event.target.files[0];

    const reader = new FileReader();

    reader.readAsArrayBuffer(file);

    reader.onloadend = function() {
      imageBytes = new Uint8Array(reader.result);

      const file = new File([imageBytes.buffer], "output.png", {type: 'image/png'});

      let resolution = get_dimensions(imageBytes);
      fabric.Image.fromURL(URL.createObjectURL(file), function (image) {
        canvas.setDimensions({width: resolution.width, height: resolution.height});
        canvas.setBackgroundImage(image, function () { initializeControls(resolution.width, resolution.height) }, {
          originX: 'left',
          originY: 'top'
        });
      });
    };
  });

  const cropButton = document.getElementById('crop-button');
  cropButton.onclick = (event) => {
    if (!imageBytes) {
      alert("Please select an image to crop.");
      return;
    }
    let croppedImage = warp_image(imageBytes,
                        coords(topLeftCircle),
                        coords(topRightCircle),
                        coords(bottomLeftCircle),
                        coords(bottomRightCircle));
    const file = new File([croppedImage.buffer], "preview.png", {type: 'image/png'});
    const croppedImageUrl = URL.createObjectURL(file)
    document.getElementById('preview').src = croppedImageUrl;
  };
})();
